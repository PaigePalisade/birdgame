use birdgame::Vector2;
use sdl2::{pixels::Color, rect::Rect, render::{Canvas, TextureCreator, TextureQuery}, ttf::Font, video::{Window, WindowContext}};

pub fn draw_text(canvas: &mut Canvas<Window>, font: &Font, pos: Vector2, centered: bool, color: Color, text: &str, texture_creator: &TextureCreator<WindowContext>) -> Result<(), String> {
    let surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())?;
    
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let TextureQuery { width, height, .. } = texture.query();

    let mut pos = pos;

    if centered {
        pos = pos - Vector2::new(width as f32 / 2.0, height as f32 / 2.0);
    }

    let target = Rect::new(
        pos.x as i32,
        pos.y as i32,
        width,
        height,
    );

    canvas.copy(&texture, None, target)?;

    Ok(())
}