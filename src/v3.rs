#[derive(Debug, Clone, Copy, PartialEq)]
pub struct V3(pub f64, pub f64, pub f64);

use std::ops;
impl ops::Add<V3> for V3 {
    type Output = V3;
    fn add(self, other: V3) -> V3 {
        let V3(x, y, z) = self;
        let V3(a, b, c) = other;
        V3(x + a, y + b, z + c)
    }
}
impl ops::Mul<V3> for V3 {
    type Output = V3;
    fn mul(self, other: V3) -> V3 {
        let V3(x, y, z) = self;
        let V3(a, b, c) = other;
        V3(x * a, y * b, z * c)
    }
}
impl ops::Mul<f64> for V3 {
    type Output = V3;
    fn mul(self, a: f64) -> V3 {
        let V3(x, y, z) = self;
        V3(x * a, y * a, z * a)
    }
}
impl ops::Sub<V3> for V3 {
    type Output = V3;
    fn sub(self, other: V3) -> V3 {
        let V3(x, y, z) = self;
        let V3(a, b, c) = other;
        V3(x - a, y - b, z - c)
    }
}
impl ops::Div<f64> for V3 {
    type Output = V3;
    fn div(self, a: f64) -> V3 {
        let V3(x, y, z) = self;
        V3(x * a, y * a, z * a)
    }
}
impl ops::Div<V3> for V3 {
    type Output = V3;
    fn div(self, other: V3) -> V3 {
        let V3(x, y, z) = self;
        let V3(a, b, c) = other;
        V3(x / a, y / b, z / c)
    }
}
impl V3 {
    pub fn len(self) -> f64 {
        self.sq_len().sqrt()
    }
    pub fn sq_len(self) -> f64 {
        let V3(x, y, z) = self;
        x * x + y * y + z * z
    }
    pub fn dot(self, other: V3) -> f64 {
        let V3(x, y, z) = self;
        let V3(a, b, c) = other;
        x * a + y * b + z * c
    }
    pub fn cross(self, other: V3) -> V3 {
        let V3(x, y, z) = self;
        let V3(a, b, c) = other;
        V3(y * c - z * b, z * a - x * c, x * b - y * a)
    }
    pub fn norm(self) -> V3 {
        self / self.len()
    }
}
