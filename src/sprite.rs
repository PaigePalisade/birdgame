pub struct Sprite<'a> {
    pub texture: &'a sdl2::render::Texture<'a>,
    pub x: f32,
    pub y: f32,
    pub scale: f32,
    pub rotation: f64,
    pub width: u32,
    pub height: u32,
    pub flip_h: bool,
    pub flip_v: bool,
}

impl<'a> Sprite<'a> {
    pub fn new(texture: &'a sdl2::render::Texture) -> Sprite<'a> {
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
            flip_h: false,
            flip_v: false,
        }
    }

    pub fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        let dest = sdl2::rect::Rect::new(
            (self.x - (self.width as f32) / 2.0 * self.scale) as i32,
            (self.y - (self.height as f32) / 2.0 * self.scale) as i32,
            (self.width as f32 * self.scale) as u32,
            (self.height as f32 * self.scale) as u32,
        );

        canvas.copy_ex(
            self.texture,
            None,
            dest,
            self.rotation,
            None,
            self.flip_h,
            self.flip_v,
        ).unwrap();
    }
}