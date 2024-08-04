use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct Tableau<T>(Vec<T>);

#[allow(dead_code)]
impl<T> Tableau<T> {
    const fn new() -> Self {
        Self(Vec::new())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn push(&mut self, item: T) {
        self.0.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}

impl<T> IntoIterator for Tableau<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Deref for Tableau<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Tableau<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tableau() {
        let mut a = Tableau::new();
        a.push(1);
        a.push(2);
        a.push(4);
        let b = a.clone();

        for it in b {
            println!("{it}");
        }

        let c: &[i32] = &*a;
        assert_eq!(c, [1, 2, 4]);
    }
}
