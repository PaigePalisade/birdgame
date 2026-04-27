mod sprite;

use std::time::Duration;

use sdl2::{event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, rect::Rect};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

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

    'running: loop {
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
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
