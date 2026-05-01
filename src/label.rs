use birdgame::Vector2;
use sdl2::{pixels::Color, rect::FRect, render::{Canvas, ScaleMode, TextureCreator, TextureQuery}, ttf::Font, video::{Window, WindowContext}};

pub fn draw_text(canvas: &mut Canvas<Window>, font: &Font, pos: Vector2, centered: bool, color: Color, text: &str, texture_creator: &TextureCreator<WindowContext>, wrap_length: u32) -> Result<(), String> {
    let surface = font
        .render(text)
        .blended_wrapped(color, wrap_length)
        .map_err(|e| e.to_string())?;
    
    let mut texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;

    texture.set_scale_mode(ScaleMode::Linear);

    let TextureQuery { width, height, .. } = texture.query();

    let mut pos = pos;

    if centered {
        pos = pos - Vector2::new(width as f32 / 4.0, height as f32 / 4.0);
    }

    let target = FRect::new(
        pos.x,
        pos.y,
        width as f32 / 2.0,
        height as f32 / 2.0,
    );

    canvas.copy_f(&texture, None, target)?;

    Ok(())
}