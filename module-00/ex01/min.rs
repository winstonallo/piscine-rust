fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn testmin() {
        assert_eq!(min(1, 2), 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(min(1, 2), 1);
    }
}