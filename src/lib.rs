use std::{f32::consts::PI, ops};

pub const RAD_TO_DEG: f64 = 180.0 / (PI as f64);

#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x, y }
    }
    pub fn lerp(&self, other: Vector2, value: f32) -> Vector2 {
        Vector2 { x: lerp(self.x, other.x, value), y: lerp(self.y, other.y, value) }
    }
    pub fn length(&self) -> f32 {
        (self.x*self.x + self.y*self.y).sqrt()
    }
    pub fn normalized(&self) -> Vector2 {
        let magn = self.length();
        // value would not be meaningful if a zero vector is passed in
        if magn == 0.0 {
            Vector2 { x: 1.0, y: 0.0 }
        } else {
            Vector2 { x: self.x / magn, y: self.y / magn }
        }
    }
    pub fn rotated(&self, theta: f64) -> Vector2 {
        Vector2 {
            x: (self.x as f64 * theta.cos() - self.y as f64 * theta.sin()) as f32,
            y: (self.x as f64 * theta.sin() + self.y as f64 * theta.cos()) as f32,
        }
    }
    pub fn angle(&self) -> f64 {
        f64::atan2(self.y as f64, self.x as f64)
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl ops::Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Vector2 {
        Vector2 { x: self.x * rhs, y: self.y * rhs }
    }
}

impl ops::Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Vector2 {
        Vector2 { x: self.x / rhs, y: self.y / rhs }
    }
}