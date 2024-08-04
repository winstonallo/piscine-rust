use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

pub struct Carton<T> {
    data: NonNull<T>,
}

impl<T> Carton<T> {
    pub fn new(val: T) -> Self {
        // `Layouy::new::<T>()` computes the size and algnment of type `T`.
        // This info is sufficient for allocating the correct amount of memory
        // for a single instance of T.
        let layout = Layout::new::<T>();

        // SAFETY:
        // Here, `alloc(layout)` has the size and layout information it needs
        // to allocate memory from the `Layout` object.
        let data = unsafe { alloc(layout) as *mut T };
        let data = NonNull::new(data).unwrap_or_else(move || handle_alloc_error(layout));

        // SAFETY:
        // Here, we are overwriting `data`, which we just allocated including handling 
        // of errors, with the content of `val`.
        unsafe { data.as_ptr().write(val) };

        Self { data }
    }

    pub fn into_inner(self) -> T {
        // SAFETY:
        // Here, we read raw memory to get the value out of the `Carton`.
        // This is safe because the memory will no longer be used after
        // this method.
        let val = unsafe { self.data.as_ptr().read() };

        // SAFETY:
        // Here, we are accessing raw memory in order to free it. This is safe
        // because this memory was allocated by this `Carton` instance and will
        // therefoe not be used anymore after this function call.
        unsafe { dealloc(self.data.as_ptr() as *mut u8, Layout::new::<T>()) };

        val
    }
}

impl<T> Drop for Carton<T> {
    fn drop(&mut self) {
        // SAFETY:
        // Here, we need to manipulate the raw pointer directly. This is 
        // safe because it will not be used anymore.
        unsafe { std::ptr::drop_in_place(self.data.as_ptr()) };

        // SAFETY:
        // Here, we are freeing the memory allocated for this `Carton` object.
        unsafe { dealloc(self.data.as_ptr() as *mut u8, Layout::new::<T>()) };
    }
}

impl<T> Deref for Carton<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // SAFETY:
        //  We control this value, and we know that it is valid.
        unsafe { self.data.as_ref() }
    }
}

impl<T> DerefMut for Carton<T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY:
        //  We control this value, and we know that it is valid.
        unsafe { self.data.as_mut() }
    }
}

impl<T: Clone> Clone for Carton<T> {
    fn clone(&self) -> Self {
        Self::new((**self).clone())
    }
}

#[test]
fn test() {
    #[derive(Clone)]
    struct Point {
        x: u32,
        y: u32,
    }
    let point_in_carton = Carton::new(Point { x: 1, y: 2 });
    assert_eq!(point_in_carton.x, 1);
    assert_eq!(point_in_carton.y, 2);

    let mut another_point = point_in_carton.clone();
    another_point.x = 2;
    another_point.y = 3;
    assert_eq!(another_point.x, 2);
    assert_eq!(another_point.y, 3);
    assert_eq!(point_in_carton.x, 1);
    assert_eq!(point_in_carton.y, 2);
}
