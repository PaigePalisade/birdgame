use std::{f64::consts::PI, ops};

pub const RAD_TO_DEG: f64 = 180.0 / PI;

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
    pub fn lerp(self, other: Vector2, value: f32) -> Vector2 {
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
    pub fn dot(self, other: Vector2) -> f32{
        self.x * other.x + self.y * other.y
    }
    pub const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };
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

// taken from Godot source code
fn angle_difference(from: f64, to: f64) -> f64 {
    let difference = (to - from) % (PI*2.0);
    (2.0 * difference) % (PI*2.0) - difference
}

pub fn rotate_toward(from: f64, to: f64, delta: f32) -> f64 {
    let difference = angle_difference(from, to);
    let abs_difference = difference.abs();
    from + (delta as f64).clamp(abs_difference - PI, abs_difference) * if difference >= 0.0 {1.0} else {-1.0}
}

// rotated rectangle intersection (I took this one from ChatGPT)

fn get_corners(center: Vector2, half: Vector2, angle: f32) -> [Vector2; 4] {
    let (s, c) = angle.sin_cos();

    let axes = [
        Vector2::new(c, s),      // x-axis rotated
        Vector2::new(-s, c),     // y-axis rotated
    ];

    let mut corners = [Vector2::ZERO; 4];
    let signs = [
        Vector2::new(-1.0, -1.0),
        Vector2::new( 1.0, -1.0),
        Vector2::new( 1.0,  1.0),
        Vector2::new(-1.0,  1.0),
    ];

    for i in 0..4 {
        corners[i] = center
            + axes[0] * half.x * signs[i].x
            + axes[1] * half.y * signs[i].y;
    }

    corners
}

fn get_axes(corners: &[Vector2; 4]) -> [Vector2; 2] {
    let edge1 = corners[1] - corners[0];
    let edge2 = corners[3] - corners[0];

    [
        edge1.normalized(),
        edge2.normalized(),
    ]
}

fn project(corners: &[Vector2; 4], axis: Vector2) -> (f32, f32) {
    let mut min = corners[0].dot(axis);
    let mut max = min;

    for &corner in corners.iter().skip(1) {
        let p = corner.dot(axis);
        min = min.min(p);
        max = max.max(p);
    }

    (min, max)
}

fn overlap(a: (f32, f32), b: (f32, f32)) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}

pub fn rectangles_intersect(
    c1: Vector2, h1: Vector2, r1: f32,
    c2: Vector2, h2: Vector2, r2: f32,
) -> bool {
    let corners1 = get_corners(c1, h1, r1);
    let corners2 = get_corners(c2, h2, r2);

    let axes1 = get_axes(&corners1);
    let axes2 = get_axes(&corners2);

    for axis in axes1.iter().chain(axes2.iter()) {
        let p1 = project(&corners1, *axis);
        let p2 = project(&corners2, *axis);

        if !overlap(p1, p2) {
            return false; // Found separating axis
        }
    }

    true // No separating axis -> collision
}