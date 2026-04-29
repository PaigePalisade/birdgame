use birdgame::{RAD_TO_DEG, Vector2, rotate_toward};

use crate::{HEIGHT, WIDTH, healthbar::draw_healthbar, sprite::Sprite};

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

    pub fn tick(&mut self, delta: f32, player_pos: Vector2) {
        self.health = self.health.max(0);
        self.timer -= delta;

        let target_rotation = (player_pos - self.pos).angle();

        self.rotation = rotate_toward(self.rotation, target_rotation, delta * 4.0);

        self.sprite.x = self.pos.x;
        self.sprite.y = self.pos.y;

        self.sprite.rotation = self.rotation * RAD_TO_DEG;
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.sprite.draw(canvas);
        draw_healthbar(canvas, self.pos, self.health);
    }
}