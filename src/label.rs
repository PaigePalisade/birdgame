use birdgame::Vector2;
use sdl2::{pixels::Color, rect::Rect, render::{Canvas, TextureCreator, TextureQuery}, ttf::Font, video::{Window, WindowContext}};

pub fn draw_text(canvas: &mut Canvas<Window>, font: &Font, pos: Vector2, color: Color, text: &str, texture_creator: &TextureCreator<WindowContext>) -> Result<(), String> {
    let surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())?;
    
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    let TextureQuery { width, height, .. } = texture.query();

    let target = Rect::new(
        pos.x as i32,
        pos.y as i32,
        width,
        height,
    );

    canvas.copy(&texture, None, target)?;

    Ok(())
}