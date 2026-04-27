use birdgame::Vector2;

use crate::sprite::Sprite;

pub struct Player<'a> {
    sprite: Sprite<'a>,
    pos: Vector2,
    vel: Vector2,
}