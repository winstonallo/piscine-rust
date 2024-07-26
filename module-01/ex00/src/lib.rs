pub fn add(a: &i32, b: i32) -> i32 {
    a + b
}

pub fn add_assign(a: &mut i32, b: i32) {
    *a += b;
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use super::*;

    #[test]
    fn testadd() {
        assert_eq!(add(2.borrow(), 2), 4)
    }

    #[test]
    fn testadd_assign() {
        let mut a: i32 = 2;
        add_assign(&mut a, 2);
        assert_eq!(a, 4);
    }
}
