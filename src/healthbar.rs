use birdgame::Vector2;
use sdl2::{pixels::Color, rect::FRect};

pub fn draw_healthbar(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, pos: Vector2, health: i32) {
    let health = health.clamp(0, 100);

    let bg_rect = FRect::new(pos.x - 12.0, pos.y + 20.0, 24.0, 4.8);

    let fg_rect= FRect::new(pos.x - 12.0, pos.y + 20.0, 24.0 * (health as f32 / 100.0), 4.8);

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.fill_frect(bg_rect).unwrap();
    if health != 0 {
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.fill_frect(fg_rect).unwrap();
    }
    canvas.set_draw_color(Color::RGB(0, 0, 0));
}