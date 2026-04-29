use birdgame::Vector2;

use crate::{HEIGHT, WIDTH, sprite::Sprite};

pub struct Enemy<'a> {
    sprite: Sprite<'a>,
    pos: Vector2,
    vel: Vector2,
    rotation: f64,
    bullet_texture: &'a sdl2::render::Texture<'a>,
    bullet_timer: f32,
    health: i32,
    timer: f32,
}

impl<'a> Enemy<'a> {
    pub fn new(texture: &'a sdl2::render::Texture, bullet_texture: &'a sdl2::render::Texture, timer: f32) -> Enemy<'a> {
        let mut out = Enemy {
            sprite: Sprite::new(texture),
            pos: Vector2::new((WIDTH as f32) / 2.0,(HEIGHT as f32) / 2.0),
            vel: Vector2::new(0.0,0.0),
            rotation: 0.0,
            bullet_texture,
            bullet_timer: 0.0,
            health: 75,
            timer,
        };
        out.sprite.scale = 2.0;

        out
    }

    fn tick(delta: f32) {

    }
}