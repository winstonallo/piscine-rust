use std::sync::atomic::{AtomicU8, Ordering};

static ID: AtomicU8 = AtomicU8::new(0);

#[derive(Debug, PartialEq, Eq)]
struct Unique(u8);

fn get_next_id() -> u8 {
    ID.fetch_add(1, Ordering::SeqCst)
}

impl Unique {
    pub fn new() -> Self {
        Self(get_next_id())
    }

    pub fn clone(&self) -> Self {
        Self(get_next_id())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn main() {
        let a = Unique::new();
        let b = Unique::new();
        let c = Unique::new();
    
        println!("{a:?}");
        println!("{b:?}");
        println!("{c:?}");
    
        let d = a.clone();
        let e = c.clone();
    
        println!("{d:?}");
        println!("{e:?}");
    }
}