use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::marker::Copy;

#[derive(Clone, Copy, Debug)]
struct Vector<T>{
    x: T,
    y: T,
}

#[allow(dead_code)]
impl Vector<f32> {
    pub fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }
}

#[allow(dead_code)]
impl Vector<f64> {
    pub fn length(&self) -> f64 {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }
}

impl<T: PartialEq + Eq> PartialEq for Vector<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T: PartialEq + Eq> Eq for Vector<T> {
    fn assert_receiver_is_total_eq(&self) {}
}

#[allow(dead_code)]
impl<T> Vector<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Eq + Add<Output = T>> Add for Vector<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Eq + Sub<Output = T>> Sub for Vector<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Eq + AddAssign> AddAssign for Vector<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Eq + SubAssign> SubAssign for Vector<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vector<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Vector<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for Vector<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Copy + DivAssign> DivAssign<T> for Vector<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn add() {
        let v1 = Vector::new(1, 2);
        let v2 = Vector::new(3, 4);
        let result = v1.add(v2);
        assert_eq!(result.x, 4);
        assert_eq!(result.y, 6);
    }

    #[test]
    fn sub() {
        let v1 = Vector::new(1, 2);
        let v2 = Vector::new(3, 4);
        let result = v1.sub(v2);
        assert_eq!(result.x, -2);
        assert_eq!(result.y, -2);
    }
    
    #[test]
    fn addassign() {
        let mut v1 = Vector::new(1, 2);
        let v2 = Vector::new(3, 4);
        v1.add_assign(v2.clone());
        assert_eq!(v1.x, 4);
        assert_eq!(v1.y, 6);
    }

    #[test]
    fn subassign() {
        let mut v1 = Vector::new(1, 2);
        let v2 = Vector::new(3, 4);
        v1.sub_assign(v2.clone());
        assert_eq!(v1.x, -2);
        assert_eq!(v1.y, -2);
    }

    #[test]
    fn mul() {
        let v1 = Vector::new(1, 2);
        let result = v1.mul(3);
        assert_eq!(result.x, 3);
        assert_eq!(result.y, 6);
    }

    #[test]
    fn div() {
        let v1 = Vector::new(8, 20);
        let result = v1.div(2);
        assert_eq!(result.x, 4);
        assert_eq!(result.y, 10);
    }

    #[test]
    fn mulassign() {
        let mut v1 = Vector::new(1, 2);
        v1.mul_assign(3);
        assert_eq!(v1.x, 3);
        assert_eq!(v1.y, 6);
    }

    #[test]
    fn divassign() {
        let mut v1 = Vector::new(8, 20);
        v1.div_assign(2);
        assert_eq!(v1.x, 4);
        assert_eq!(v1.y, 10);
    }

    #[test]
    fn test_a() {
        let v = Vector {
            x: String::from("Hello, World!"),
            y: String::from("Hello, Rust!"),
        };

        let w = v.clone();

        assert_eq!(&v, &w);
    }

    #[test]
    fn test_b() {
        let v = Vector::new("Hello, World!", "Hello, Rust!");
        let a = v;
        let b = v;
        assert_eq!(a, b);
    }
}