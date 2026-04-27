struct Sprite<'a> {
    texture: sdl2::render::Texture<'a>,
    x: f32,
    y: f32,
    scale: f32,
    rotation: f64,
    width: u32,
    height: u32,
}

impl<'a> Sprite<'a> {
    fn new(texture: sdl2::render::Texture<'a>) -> Sprite<'a> {
        let query = texture.query();
        let width = query.width;
        let height = query.height;
        Sprite {
            texture,
            x: 0.0,
            y: 0.0,
            scale: 1.0,
            rotation: 0.0,
            width,
            height,
        }
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let dest = sdl2::rect::Rect::new(
            self.x as i32,
            self.y as i32,
            (self.width as f32 * self.scale) as u32,
            (self.height as f32 * self.scale) as u32,
        );

        canvas.copy_ex(
            &self.texture,
            None,
            dest,
            self.rotation,
            None,
            false,
            false,
        ).unwrap();
    }
}