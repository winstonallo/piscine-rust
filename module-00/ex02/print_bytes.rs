fn print_bytes(s: &str) {
    for b in s.bytes() {
        println!("{}", b);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testprintbytes() {
        print_bytes("arthur");
    }
}