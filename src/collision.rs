use birdgame::Vector2;

use crate::{bullet::Bullet, enemy::Enemy, player::Player};

pub struct CollisionRect {
    pub center: Vector2,
    pub rotation: f64,
    half_extents: Vector2,
}

impl CollisionRect {
    pub fn new(center: Vector2, width: f32, height: f32, rotation: f64) -> CollisionRect {
        CollisionRect {
            center,
            half_extents: Vector2::new(width/2.0, height/2.0),
            rotation,
        }
    }
    fn is_intersecting_with(&self, other: &CollisionRect) -> bool {
        rectangles_intersect(
            self.center,
            self.half_extents,
            self.rotation as f32,
            other.center,
            other.half_extents,
            other.rotation as f32,
        )
    }
}

pub fn player_bullets_collision(player: &mut Player, enemy_bullets: &mut Vec<Bullet>) {
    for bullet in enemy_bullets {
        if bullet.bounding_box.is_intersecting_with(&player.bounding_box) {
            player.health -= 5;
            bullet.dead = true;
        }
    }
}

pub fn enemy_bullets_collision(enemies: &mut Vec<Enemy>, player_bullets: &mut Vec<Bullet>, score: &mut i32, player_health: &mut i32) {
    for bullet in player_bullets {
        for enemy in &mut *enemies {
            if enemy.health > 0 && bullet.bounding_box.is_intersecting_with(&enemy.bounding_box) {
                enemy.health -= 40;
                bullet.dead = true;
                *score += 10;
                if enemy.health <= 0 && *player_health > 0 {
                    *player_health = (*player_health + 5).min(100);
                    *score += 100;
                }
            }
        }
    }
}

pub fn player_enemy_collision(enemies: &mut Vec<Enemy>, player: &mut Player) {
    for enemy in enemies {
        if enemy.health > 0 && player.bounding_box.is_intersecting_with(&enemy.bounding_box) {
            enemy.health = 0;
            player.health -= 10;
        }
    }
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

fn rectangles_intersect(
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