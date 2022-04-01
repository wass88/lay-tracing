#[derive(Debug, Clone, Copy, PartialEq)]
pub struct V3(pub f64, pub f64, pub f64);
pub static PI: f64 = std::f64::consts::PI;

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
        V3(x / a, y / a, z / a)
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
impl ops::Neg for V3 {
    type Output = V3;
    fn neg(self) -> V3 {
        self * -1.
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
    pub fn near_zero(self) -> bool {
        self.sq_len() < 1e-16
    }
    pub fn reflect(self, normal: V3) -> V3 {
        self - normal * self.dot(normal) * 2.
    }
    pub fn refract(self, normal: V3, etai: f64) -> V3 {
        let cos = -self.dot(normal);
        let prep = (self + normal * cos) * etai;
        let para = -normal * (1. - prep.sq_len()).sqrt();
        prep + para
    }
}

pub fn rand_in(min: f64, max: f64) -> f64 {
    use crate::rand::distributions::Distribution;
    rand::distributions::Uniform::from(min..max).sample(&mut rand::thread_rng())
}
pub fn rand() -> f64 {
    rand_in(0., 1.)
}
pub fn rand_v3() -> V3 {
    let phi = rand_in(0., 2. * PI);
    let ctheta = rand_in(-1., 1.);
    let theta = ctheta.acos();
    let x = theta.sin() * phi.cos();
    let y = theta.sin() * phi.sin();
    let z = theta.cos();
    V3(x, y, z)
}
pub fn rand_hemisphere(normal: V3) -> V3 {
    let v = rand_v3();
    if normal.dot(v) > 0. {
        v
    } else {
        -v
    }
}
pub fn rand_disk() -> V3 {
    let r = rand().sqrt();
    let theta = rand() * 2.0 * PI;
    V3(r * theta.cos(), r * theta.sin(), 0.)
}
