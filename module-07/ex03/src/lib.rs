use std::{cell::UnsafeCell, clone::Clone, marker::Copy};

#[allow(dead_code)]
struct Cellule<T> {
    cell: UnsafeCell<T>,
}

#[allow(dead_code)]
impl<T: Copy> Cellule<T> {
    const fn new(value: T) -> Self {
        Self {
            cell: UnsafeCell::new(value),
        }
    }

    fn set(&mut self, value: T) {
        let ptr = self.cell.get_mut();
        *ptr = value;
    }

    fn replace(&mut self, value: T) -> T {
        let ptr = self.cell.get_mut();
        std::mem::replace(ptr, value)
    }

    fn get(&self) -> T {
        // SAFETY:
        // This is safe because we return a copy of the data,
        // which cannot be mutated by the caller.
        unsafe { *self.cell.get().clone() }
    }

    fn get_mut(&mut self) -> &mut T {
        self.cell.get_mut()
    }

    fn into_inner(self) -> T {
        self.cell.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut cell = Cellule::new(42);

        assert_eq!(cell.get(), 42);

        cell.set(100);
        assert_eq!(cell.get(), 100);

        let old_value = cell.replace(200);
        assert_eq!(old_value, 100);
        assert_eq!(cell.get(), 200);

        let value = cell.into_inner();
        assert_eq!(value, 200);
    }
}
