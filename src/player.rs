use birdgame::{RAD_TO_DEG, Vector2};

use crate::{HEIGHT, SCALE, WIDTH, bullet::Bullet, healthbar::draw_healthbar, sprite::Sprite};

pub struct Player<'a> {
    sprite: Sprite<'a>,
    pub pos: Vector2,
    vel: Vector2,
    rotation: f64,
    bullet_texture: &'a sdl2::render::Texture<'a>,
    bullet_timer: f32,
    health: i32,
}

impl<'a> Player<'a> {
    pub fn new(texture: &'a sdl2::render::Texture, bullet_texture: &'a sdl2::render::Texture) -> Player<'a> {
        let mut out = Player {
            sprite: Sprite::new(texture),
            pos: Vector2::new((WIDTH as f32) / 2.0,(HEIGHT as f32) / 2.0),
            vel: Vector2::new(0.0,0.0),
            rotation: 0.0,
            bullet_texture,
            bullet_timer: 0.0,
            health: 75,
        };
        out.sprite.scale = 2.0;

        out
    }
    
    pub fn tick(&mut self, delta: f32, e: &sdl2::EventPump, player_bullets: &mut Vec<Bullet<'a>>) {
        let mouse_state = e.mouse_state();
        let mouse_pos = Vector2::new(mouse_state.x() as f32, mouse_state.y() as f32) / SCALE;
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
            let bullet = Bullet::new(self.bullet_texture, bullet_pos, bullet_vel, bullet_vel.angle() as f64);
            player_bullets.push(bullet);
            self.bullet_timer = 0.05;
        }

        self.pos = self.pos + self.vel * delta;

        self.bullet_timer -= delta;

        self.sprite.x = self.pos.x;
        self.sprite.y = self.pos.y;

        self.sprite.rotation = self.rotation * RAD_TO_DEG;
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.sprite.draw(canvas);
        draw_healthbar(canvas, self.pos, self.health);
    }
}