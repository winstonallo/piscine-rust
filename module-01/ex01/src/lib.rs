pub fn min(a: &i32, b: &i32) -> i32 {
    match *a < *b {
        true => *a,
        false => *b,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = min(&2, &2);
        assert_eq!(result, 2);
    }
}
