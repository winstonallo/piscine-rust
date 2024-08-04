#![forbid(unsafe_op_in_unsafe_fn)]

/// ## SAFETY:
/// 1. Passing invalid (null/dangling) pointers to `std::ptr::read` and
/// `std::ptr::write` will result in invalid memory access.
/// 2. This function operates on geeneric types: types with complex
/// invariants might be broken by raw pointer manipulation.
pub fn ft_swap<T>(a: &mut T, b: &mut T) {
    // SAFETY:
    // Here, we assume that `a` and `b` are valid pointers.
    unsafe {
        let tmp = std::ptr::read(a);

        std::ptr::write(a, std::ptr::read(b));

        std::ptr::write(b, tmp);
    }
}

/// ## SAFETY
/// It is the caller's responsibility to ensure that `s` is a valid pointer
/// to a properly null-terminated u8 array. Failure to do so will result
/// in invalid memory access.
pub unsafe fn ft_strlen(s: *const u8) -> usize {
    let mut size = 0;

    // SAFETY:
    // Here, we assume that `s` is null-terminated.
    while unsafe { s.add(size).read() } != 0 {
        size += 1;
    }
    size
}

/// ## SAFETY
/// It is the caller's repsonsibility to ensure that `dst` is a valid pointer
/// to a buffer of size `>= ft_strlen(src) + 1` bytes. `src` must be a valid pointer
/// to a properly null-terminated string.
pub unsafe fn ft_strcpy(dst: *mut u8, src: *const u8) {
    let mut idx = 0;

    // SAFETY:
    // Here, we assume that `dst` is of size `>= ft_strlen(src)` and
    // that `src` is null-terminated.
    while unsafe { src.add(idx).read() } != 0 {
        unsafe { dst.add(idx).write(src.add(idx).read()) };
        idx += 1;
    }

    // SAFETY:
    // Here, we assume that `dst` is of size `>= ft_strlen(src) + 1`.
    unsafe { dst.add(idx).write(b'\0') };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn swap_basic_type() {
        let mut a = 69;
        let mut b = 420;
        ft_swap(&mut a, &mut b);
        assert_eq!(a, 420);
        assert_eq!(b, 69);
    }

    #[test]
    fn swap_complex_type() {
        let mut a = String::from("Hello, World!");
        let mut b = String::from("Goodbye, World!");
        ft_swap(&mut a, &mut b);
        assert_eq!(a, "Goodbye, World!");
        assert_eq!(b, "Hello, World!");
    }

    #[test]
    fn strlen() {
        let s = b"Hello, World!\0";
        let len = unsafe { ft_strlen(s.as_ptr()) };
        assert_eq!(len, 13);
    }

    #[test]
    fn strcpy() {
        let s = b"Hello, World!\0";
        let mut dst = [0u8; 14];
        unsafe { ft_strcpy(dst.as_mut_ptr(), s.as_ptr()) };
        assert_eq!(&dst, s);
    }
}
