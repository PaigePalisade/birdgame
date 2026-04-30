use std::f64::consts::PI;

use birdgame::{RAD_TO_DEG, Vector2, angle_difference, rotate_toward};
use sdl2::render::Texture;

use crate::{HEIGHT, WIDTH, bullet::Bullet, collision::CollisionRect, healthbar::draw_healthbar, sprite::Sprite};

#[derive(PartialEq)]
enum EnemyState {
    DEAD,
    SHOOTING,
    HOMING,
    RESTING,
}

use EnemyState::*;

pub struct Enemy<'a> {
    sprite: Sprite<'a>,
    pos: Vector2,
    rotation: f64,
    default_texture: &'a Texture<'a>,
    explosion_texture: &'a Texture<'a>,
    bullet_texture: &'a Texture<'a>,
    bullet_timer: f32,
    pub health: i32,
    timer: f32,
    explosion_timer: f32,
    state: EnemyState,
    pub bounding_box: CollisionRect,
}

impl<'a> Enemy<'a> {
    pub fn new(texture: &'a Texture, explosion_texture: &'a Texture, bullet_texture: &'a Texture, timer: f32) -> Enemy<'a> {
        let mut out = Enemy {
            sprite: Sprite::new(texture),
            pos: Vector2::new((WIDTH as f32) / 2.0,(HEIGHT as f32) / 2.0),
            rotation: 0.0,
            bullet_texture,
            default_texture: texture,
            explosion_texture: explosion_texture,
            bullet_timer: 0.0,
            health: 0,
            timer,
            explosion_timer: 0.0,
            state: RESTING,
            bounding_box: CollisionRect::new(Vector2::ZERO, 40.0, 24.0, 0.0),
        };
        out.sprite.scale = 2.0;

        out
    }

    pub fn tick(&mut self, delta: f32, player_pos: Vector2, enemy_bullets: &mut Vec<Bullet<'a>>) {
        self.health = self.health.max(0);
        self.timer -= delta;

        
        if self.health <= 0 && self.state != DEAD && self.state != RESTING {
            self.state = DEAD;
            self.explosion_timer = 0.2;
            self.timer = 12.0;
        }
        
        match self.state {
            RESTING => {
                if self.timer < 10.0 {
                    if self.state == RESTING {
                        self.pos = player_pos;
                        while (self.pos - player_pos).length() < 200.0 {
                            self.pos = Vector2 { 
                                x: rand::random_range(0.0..(WIDTH as f32)),
                                y: rand::random_range(0.0..(HEIGHT as f32)),
                            }
                        }
                    }
                    self.state = SHOOTING;
                    self.health = 100;
                }
            }
            SHOOTING => {
                let target_rotation = (player_pos - self.pos).angle();
                self.rotation = rotate_toward(self.rotation, target_rotation, delta * 4.0);

                if self.bullet_timer < 0.0 {
                    let bullet_vel = Vector2::new(1.0,0.0).rotated(self.rotation) * 500.0;
                    let bullet_pos = self.pos + bullet_vel.normalized() * 20.0;
                    let bullet = Bullet::new(self.bullet_texture, bullet_pos, bullet_vel, bullet_vel.angle());
                    enemy_bullets.push(bullet);
                    self.bullet_timer = 0.2;
                }

                if self.timer < 5.0 {
                    self.state = HOMING
                }

                self.bullet_timer -= delta;
            },
            HOMING => {
                let target_rotation = (player_pos - self.pos).angle();
                self.rotation = rotate_toward(self.rotation, target_rotation, delta * 2.0);
                self.pos = self.pos + Vector2::new(1.0,0.0).rotated(self.rotation) * delta * 500.0;
            },
            DEAD => {
                self.explosion_timer -= delta;
                if self.explosion_timer < 0.0 {
                    self.state = RESTING;
                }
            },
        }

        self.sprite.x = self.pos.x;
        self.sprite.y = self.pos.y;
        self.sprite.flip_v = angle_difference(self.rotation, 0.0).abs() >= PI / 2.0;
        self.sprite.rotation = self.rotation * RAD_TO_DEG;
        self.sprite.texture = if self.state == DEAD {self.explosion_texture} else {self.default_texture};

        self.bounding_box.center = self.pos;
        self.bounding_box.rotation = self.rotation;
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        if self.state != RESTING {
            self.sprite.draw(canvas);
            draw_healthbar(canvas, self.pos, self.health);
        }
    }
}