use ftkit::random_number;

fn choose<T>(values: &[T]) -> &T {
    if values.is_empty() {
        panic!("the container is empty");
    }

    let index = random_number(0..values.len() as i32) as usize;

    &values[index]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn panic_test() {
        let v: Vec<i32> = vec![];
        choose(&v);
    }

    #[test]
    fn standard_test() {
        let v = vec![0, 1, 2, 3, 4];
        let result = choose(&v);
        assert!(v.contains(result));
    }
}

fn main() {}
