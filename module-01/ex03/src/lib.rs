pub fn largest_group<'a>(haystack: &'a [u32], needle: &[u32]) -> &'a [u32] {
    let mut largest: &[u32] = &[];

    for start in 0..haystack.len() {
        let mut end = start;
        while end < haystack.len() && needle.contains(&haystack[end]) {
            end += 1;
        }
        let candidate = &haystack[start..end];
        let mut validated = true;
        for item in needle {
            if !candidate.contains(item) {
                validated = false;
                break;
            }
        }
        if !validated {
            continue;
        }
        if largest.len() < candidate.len() {
            largest = candidate;
        }
    }
    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifetimes() {
        let haystack = [1, 2, 3, 2, 1];
        let result;

        {
            let needle = [2, 3];
            result = largest_group(&haystack, &needle);
        }

        assert_eq!(result, &[2, 3, 2]);
    }

    #[test]
    fn test_examples() {
        assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5, 3]), &[3, 5, 5]);
        assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[5]), &[5, 5]);
        assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[]), &[]);
        assert_eq!(largest_group(&[1, 3, 4, 3, 5, 5, 4], &[4, 1]), &[]);
    }
}
