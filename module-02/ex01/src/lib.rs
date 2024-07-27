pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    pub fn zero() -> Self {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn distance(&self, other: &Self) -> f32 {
        ((self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)).sqrt()
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.x += dx;
        self.y += dy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let point = Point::new(6.9, 42.0);
        assert_eq!(point.x, 6.9);
        assert_eq!(point.y, 42.0);
    }

    #[test]
    fn zero() {
        let point = Point::zero();
        assert_eq!(point.x, 0.0);
        assert_eq!(point.y, 0.0);
    }

    #[test]
    fn distance() {
        let p1 = Point::zero();
        let p2 = Point::new(6.9, 42.0);
        let distance = p1.distance(&p2);
        assert_eq!(distance, 42.563012111);
    }

    #[test]
    fn translate() {
        let mut p1 = Point::zero();
        let p2 = Point::new(6.9, 42.0);
        p1.translate(p2.x, p2.y);
        assert_eq!(p1.x, p2.x);
        assert_eq!(p1.y, p2.y);
    }
}
