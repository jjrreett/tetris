#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x, y }
    }
}

use std::ops::{Add, Mul};

impl<T> Add for Vec2<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Vec2<T>;

    fn add(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Vec2<T>;

    fn mul(self, scalar: T) -> Vec2<T> {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// Tests...
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_addition_i32() {
        let p1 = Vec2::new(1i32, 2i32);
        let p2 = Vec2::new(3i32, 4i32);
        let expected = Vec2::new(4i32, 6i32);
        assert_eq!(p1 + p2, expected);
    }

    #[test]
    fn test_point_addition_f64() {
        let p1 = Vec2::new(1.0f64, 2.0f64);
        let p2 = Vec2::new(3.0f64, 4.0f64);
        let expected = Vec2::new(4.0f64, 6.0f64);
        assert_eq!(p1 + p2, expected);
    }

    #[test]
    fn test_point_multiplication_i32() {
        let p = Vec2::new(1i32, 2i32);
        let expected = Vec2::new(2i32, 4i32);
        assert_eq!(p * 2i32, expected);
    }

    #[test]
    fn test_point_multiplication_f64() {
        let p = Vec2::new(1.0f64, 2.0f64);
        let expected = Vec2::new(2.0f64, 4.0f64);
        assert_eq!(p * 2.0f64, expected);
    }

    // Additional tests for different types...
}
