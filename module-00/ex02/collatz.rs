fn collatz(start: u32) {
    if start == 0 {
        return;
    }
    let mut n: u32 = start;
    while n != 1 {
        println!("{}",n);
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testcollatz() {
        collatz(70);
    }
}

fn main() {
    collatz(0);
}