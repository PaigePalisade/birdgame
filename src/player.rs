use birdgame::{RAD_TO_DEG, Vector2};

use crate::{HEIGHT, WIDTH, sprite::{GameObj, Sprite}};

pub struct Player<'a> {
    sprite: Sprite<'a>,
    pos: Vector2,
    vel: Vector2,
    rotation: f64,
}

impl<'a> Player<'a> {
    pub fn new(texture: &'a sdl2::render::Texture) -> Player<'a> {
        let mut out = Player {
            sprite: Sprite::new(texture),
            pos: Vector2::new((WIDTH as f32) / 2.0,(HEIGHT as f32) / 2.0),
            vel: Vector2::new(0.0,0.0),
            rotation: 0.0,
        };
        out.sprite.scale = 2.0;

        out
    }
}

impl<'a> GameObj for Player<'a> {
    fn tick(&mut self, delta: f32, e: &sdl2::EventPump) {
        let mouse_state = e.mouse_state();
        let mouse_pos = Vector2::new(mouse_state.x() as f32, mouse_state.y() as f32);
        let mouse_diff = mouse_pos - self.pos;
        self.rotation = f64::atan2(mouse_diff.y as f64, mouse_diff.x as f64);

        self.vel = self.vel.lerp(mouse_diff * 5.0, delta);

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


        self.pos = self.pos + self.vel * delta;

        self.sprite.x = self.pos.x;
        self.sprite.y = self.pos.y;

        self.sprite.rotation = self.rotation * RAD_TO_DEG;
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        self.sprite.draw(canvas);
    }

}