pub fn deduplicate(list: &mut Vec<i32>) {
    let mut i = 0;
    while i < list.len() {
        let mut j = i + 1;
        while j < list.len() {
            if list[i] == list[j] {
                list.remove(j);
            } else {
                j += 1;
            }
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut v = vec![1, 2, 2, 3, 2, 4, 3];
        deduplicate(&mut v);
        assert_eq!(v, [1, 2, 3, 4]);
    }
}
