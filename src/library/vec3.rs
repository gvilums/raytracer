use std::ops::{Add, Sub};
use std::fmt::Debug;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 {x, y, z}
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length(&self) -> f64 {
        self.dot(&self).sqrt()
    }

    pub fn dot_self(&self) -> f64 {
        self.dot(&self)
    }

    pub fn scale(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }

    pub fn norm(&mut self) {
        self.scale(1. / self.length());
    }

    pub fn scaled(&self, factor: f64) -> Self {
        let mut out = self.clone();
        out.scale(factor);
        return out;
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}