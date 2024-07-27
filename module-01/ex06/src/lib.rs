pub fn big_add(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut result = Vec::new();
    let mut carry = 0;

    let mut i = a.len();
    let mut j = b.len();

    while i > 0 || j > 0 || carry > 0 {
        let digit_a = if i > 0 {
            i -= 1;
            (a[i] - b'0') as u8
        } else {
            0
        };
        let digit_b = if j > 0 {
            j -= 1;
            (b[j] - b'0') as u8
        } else {
            0
        };
        let sum = digit_a + digit_b + carry;
        carry = sum / 10;
        result.push((sum % 10) + b'0');
    }
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = big_add(b"21", b"21");
        assert_eq!(result, b"42");
    }

    #[test]
    #[should_panic]
    fn its_broken() {
        let result = big_add(b"a21", b"21");
        assert_eq!(result, b"42");
    }
}
