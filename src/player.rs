use birdgame::{RAD_TO_DEG, Vector2};
use sdl2::{rect::Rect, render::Texture};

use crate::{HEIGHT, WIDTH, bullet::Bullet, collision::CollisionRect, healthbar::draw_healthbar, sprite::Sprite};

pub struct Player<'a> {
    sprite: Sprite<'a>,
    pub pos: Vector2,
    vel: Vector2,
    rotation: f64,
    explosion_texture: &'a Texture<'a>,
    bullet_texture: &'a Texture<'a>,
    bullet_timer: f32,
    pub health: i32,
    pub bounding_box: CollisionRect,
    pub explosion_timer: f32,
}

impl<'a> Player<'a> {
    pub fn new(texture: &'a Texture, explosion_texture: &'a Texture, bullet_texture: &'a Texture) -> Player<'a> {
        let mut out = Player {
            sprite: Sprite::new(texture),
            pos: Vector2::new((WIDTH as f32) / 2.0,(HEIGHT as f32) / 2.0),
            vel: Vector2::new(0.0,0.0),
            rotation: 0.0,
            explosion_texture,
            bullet_texture,
            bullet_timer: 0.0,
            health: 100,
            bounding_box: CollisionRect::new(Vector2::ZERO, 40.0, 24.0, 0.0),
            explosion_timer: 0.2,
        };
        out.sprite.scale = 2.0;

        out
    }
    
    pub fn tick(&mut self, delta: f32, e: &sdl2::EventPump, player_bullets: &mut Vec<Bullet<'a>>, display_scale: f32, display_rect: Rect) {
        if self.health <= 0 {
            self.explosion_timer -= delta;
            self.sprite.texture = self.explosion_texture;
            return;
        }
        let mouse_state = e.mouse_state();
        let mouse_pos = Vector2::new(mouse_state.x() as f32, mouse_state.y() as f32) / display_scale - Vector2::new(display_rect.x as f32, display_rect.y as f32);
        let mouse_diff = mouse_pos - self.pos;
        self.rotation = f64::atan2(mouse_diff.y as f64, mouse_diff.x as f64);

        self.vel = Vector2::lerp(self.vel, mouse_diff * 5.0, delta);

        self.sprite.flip_v = mouse_diff.x < 0.0;

        if self.pos.x < 0.0 {
            self.pos.x = 0.0;
            self.vel.x = 0.0;
        }
        if self.pos.x > WIDTH as f32 {
            self.pos.x = WIDTH as f32;
            self.vel.x = 0.0;
        }
        if self.pos.y < 0.0 {
            self.pos.y = 0.0;
            self.vel.y = 0.0;
        }
        if self.pos.y > HEIGHT as f32 {
            self.pos.y = HEIGHT as f32;
            self.vel.y = 0.0;
        }

        if mouse_state.is_mouse_button_pressed(sdl2::mouse::MouseButton::Left) && self.bullet_timer < 0.0 {
            let bullet_vel = self.vel + Vector2::new(1.0, 0.0).rotated(self.rotation) * 800.0;
            let bullet_pos = self.pos + bullet_vel.normalized() * 20.0;
            let bullet = Bullet::new(self.bullet_texture, bullet_pos, bullet_vel, bullet_vel.angle());
            player_bullets.push(bullet);
            self.bullet_timer = 0.05;
        }

        self.pos = self.pos + self.vel * delta;

        self.bullet_timer -= delta;

        self.sprite.x = self.pos.x;
        self.sprite.y = self.pos.y;
        self.sprite.rotation = self.rotation * RAD_TO_DEG;

        self.bounding_box.center = self.pos;
        self.bounding_box.rotation = self.rotation;
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.sprite.draw(canvas);
        draw_healthbar(canvas, self.pos, self.health);
    }
}