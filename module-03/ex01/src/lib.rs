fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(min(12i32, -14i32), -14);
        assert_eq!(min(12f32, 14f32), 12f32);
        assert_eq!(min("abc", "def"), "abc");
        assert_eq!(min(String::from("abc"), String::from("def")), "abc");
    }
}
