fn fits(a: [u32; 2], b: [u32; 2]) -> bool {
    a[0] <= b[0] && a[1] <= b[1]
}

fn biggest_box(boxes: &[[u32; 2]]) -> usize {
    let mut max = 0;

    for i in 1..boxes.len() {
        if fits(boxes[max], boxes[i]) {
            max = i;
        }
    }
    max
}

pub fn sort_boxes(boxes: &mut [[u32; 2]]) {
    let mut temp: &mut [[u32; 2]] = &mut boxes[..];

    while !temp.is_empty() {
        let max = biggest_box(temp);
        temp.swap(0, max);
        temp = &mut temp[1..];
    }
    for i in 0..boxes.len() - 1 {
        assert!(fits(boxes[i + 1], boxes[i]));
    }
}

#[cfg(test)]
#[test]
#[should_panic]
fn test_impossible() {
    let mut boxes = [[1, 2], [2, 1]];
    sort_boxes(&mut boxes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut boxes = [[3, 3], [4, 3], [1, 0], [5, 7], [3, 3]];
        sort_boxes(&mut boxes);
        assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
    }
}
