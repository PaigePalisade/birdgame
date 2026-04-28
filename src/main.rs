mod sprite;
mod player;

use std::{time::{Instant}};

use sdl2::{event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, rect::Rect};

use crate::{player::Player, sprite::GameObj};

pub const WIDTH: u32 = 1280;
pub const HEIGHT: u32 = 720;

fn main() -> Result<(), String>{
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Bird Wars", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    let texture_creator = canvas.texture_creator();
    let sky_texture = texture_creator.load_texture("assets/textures/sky.png")?;
    let player_texture = texture_creator.load_texture("assets/textures/player.png")?;

    let mut player = Player::new(&player_texture);

    let mut last_frame = Instant::now();
    let mut delta;

    'running: loop {
        delta = ((Instant::now() - last_frame).as_nanos() as f32) / 1_000_000_000f32;
        last_frame = Instant::now();
        player.tick(delta, &event_pump);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        canvas.clear();
        canvas.copy(&sky_texture, None, Rect::new(0,0,WIDTH,HEIGHT))?;
        player.draw(&mut canvas);
        canvas.present();
        println!("FPS: {}", 1.0 / delta);
    }

    Ok(())
}
