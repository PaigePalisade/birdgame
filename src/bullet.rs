use birdgame::{RAD_TO_DEG, Vector2};

use crate::{HEIGHT, WIDTH, collision::CollisionRect, sprite::Sprite};

pub struct Bullet<'a> {
    sprite: Sprite<'a>,
    pos: Vector2,
    vel: Vector2,
    rotation: f64,
    pub dead: bool,
    pub bounding_box: CollisionRect,
}

impl<'a> Bullet<'a> {
    pub fn new(texture: &'a sdl2::render::Texture, pos: Vector2, vel: Vector2, rotation: f64) -> Bullet<'a> {
        let mut out = Bullet { 
            sprite: Sprite::new(texture),
            pos,
            vel,
            rotation,
            dead: false,
            bounding_box: CollisionRect::new(Vector2::ZERO, 14.0, 6.0, 0.0),
        };
        out.sprite.scale = 2.0;
        out
    }

    pub fn tick(&mut self, delta: f32) {
        self.pos = self.pos + self.vel * delta;
        if self.vel.length() < 500.0 {
            self.vel = self.vel + self.vel.normalized() * 500.0 * delta;
        }
        if  self.pos.x < -100.0 || self.pos.y < -100.0 || self.pos.x > WIDTH as f32 + 100.0 || self.pos.y > HEIGHT as f32 + 100.0 {
            self.dead = true;
        }

        self.sprite.x = self.pos.x;
        self.sprite.y = self.pos.y;
        self.sprite.rotation = self.rotation * RAD_TO_DEG;

        self.bounding_box.center = self.pos;
        self.bounding_box.rotation = self.rotation;
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        if !self.dead {
            self.sprite.draw(canvas);
        }
    }
}