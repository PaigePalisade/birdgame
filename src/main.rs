mod sprite;
mod player;
mod bullet;
mod healthbar;
mod enemy;
mod collision;
mod label;

use std::time::Instant;

use birdgame::Vector2;
use sdl2::{EventPump, event::Event, image::LoadTexture, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, ttf::Sdl2TtfContext, video::{FullscreenType, Window}};

use crate::{bullet::Bullet, collision::{enemy_bullets_collision, player_bullets_collision, player_enemy_collision}, enemy::Enemy, label::draw_text, player::Player};

pub const WIDTH: u32 = 1280;
pub const HEIGHT: u32 = 720;

const MAX_FPS: f32 = 500.0;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("Bird Wars", (WIDTH as f32 ) as u32, (HEIGHT as f32) as u32)
        .position_centered()
        .opengl()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas: Canvas<sdl2::video::Window> = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    let mut score = 0;
    let mut play_again = true;
    while play_again {

        play_again = welcome(&mut canvas, &mut event_pump, &mut score, &ttf_context)? &&
                     game(&mut canvas, &mut event_pump, &mut score, &ttf_context)?;
    }
    
    Ok(())
}

fn welcome<'a>(canvas: &mut Canvas<Window>, event_pump: &mut EventPump, score: &mut i32, ttf_context: &Sdl2TtfContext) -> Result<bool, String> {
    let texture_creator = canvas.texture_creator();
    let sky_texture = texture_creator.load_texture("assets/textures/sky.png")?;
    let pixel_font = ttf_context.load_font("assets/fonts/Kenney Pixel.ttf", 128)?;
    let future_font = ttf_context.load_font("assets/fonts/Kenney Future Narrow.ttf", 192)?;
    let sans_font = ttf_context.load_font("assets/fonts/OpenSans.ttf", 64)?;

    loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Ok(false);
                },
                Event::KeyDown { keycode: Some(Keycode::F11), .. } => {
                    let window = canvas.window_mut(); 
                    window.set_fullscreen(
                        if window.fullscreen_state() == FullscreenType::Desktop {
                            FullscreenType::Off
                        } else {
                            FullscreenType::Desktop
                        }
                    )?;
                },
                Event::MouseButtonDown { .. } => {
                    return Ok(true)
                },
                _ => {}
            }
        }

        let (window_width, window_height) = canvas.output_size()?;
        let display_scale =
        if window_width as f32 / window_height as f32 > 16.0/9.0 {
            window_height as f32 / HEIGHT as f32
        }
        else {
            window_width as f32 / WIDTH as f32
        };

        canvas.set_scale(display_scale, display_scale)?;
        canvas.set_viewport(Rect::new(
            ((window_width as f32 - WIDTH as f32 * display_scale) / 2.0 / display_scale) as i32,
            ((window_height as f32 - HEIGHT as f32 * display_scale) / 2.0 / display_scale) as i32,
            WIDTH,
            HEIGHT,
        ));

        canvas.clear();
        canvas.copy(&sky_texture, None, Rect::new(0,0,WIDTH,HEIGHT))?;
        
        draw_text(canvas,
            &pixel_font,
            Vector2::new(
            20.0, HEIGHT as f32 - 64.0),
            false,
            Color::RGB(0, 0, 0),
            &format!("Score: {}", score),
            &texture_creator,
            WIDTH,
        )?;

        draw_text(canvas,
            &future_font,
            Vector2::new(
            WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0 - 100.0),
            true,
            Color::RGB(27, 33, 52),
            " Bird Wars",
            &texture_creator,
            WIDTH,
        )?;
        
        draw_text(canvas,
            &sans_font,
            Vector2::new(
            WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
            true,
            Color::RGB(27, 33, 52),
            "Click Anywhere to Play!",
            &texture_creator,
            WIDTH,
        )?;

        canvas.present();
    }
}

fn game<'a>(canvas: &mut Canvas<Window>, event_pump: &mut EventPump, score: &mut i32, ttf_context: &Sdl2TtfContext) -> Result<bool, String> {
    let texture_creator = canvas.texture_creator();
    let sky_texture = texture_creator.load_texture("assets/textures/sky.png")?;
    let player_texture = texture_creator.load_texture("assets/textures/player.png")?;
    let player_bullet_texture = texture_creator.load_texture("assets/textures/bullet.png")?;
    let enemy_texture = texture_creator.load_texture("assets/textures/enemy.png")?;
    let enemy_bullet_texture = texture_creator.load_texture("assets/textures/evilbullet.png")?;
    let explosion_texture = texture_creator.load_texture("assets/textures/explosion.png")?;

    let pixel_font = ttf_context.load_font("assets/fonts/Kenney Pixel.ttf", 128)?;

    let mut player_bullets: Vec<Bullet> = vec![];
    let mut player = Player::new(&player_texture, &explosion_texture, &player_bullet_texture);

    let mut enemy_bullets: Vec<Bullet> = vec![];
    let mut enemies = vec![
        Enemy::new(&enemy_texture, &explosion_texture, &enemy_bullet_texture, 12.0),
        Enemy::new(&enemy_texture, &explosion_texture, &enemy_bullet_texture, 30.0),
        Enemy::new(&enemy_texture, &explosion_texture, &enemy_bullet_texture, 60.0),
        Enemy::new(&enemy_texture, &explosion_texture, &enemy_bullet_texture, 70.0),
        Enemy::new(&enemy_texture, &explosion_texture, &enemy_bullet_texture, 200.0),
        Enemy::new(&enemy_texture, &explosion_texture, &enemy_bullet_texture, 210.0),
    ];

    let mut last_frame = Instant::now();
    let mut delta;

    let mut score_timer = 1.0f32;
    *score = 0;

    loop {
        delta = ((Instant::now() - last_frame).as_nanos() as f32) / 1_000_000_000f32;
        last_frame = Instant::now();
        score_timer -= delta;
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return Ok(false);
                },
                Event::KeyDown { keycode: Some(Keycode::F11), .. } => {
                    let window = canvas.window_mut(); 
                    window.set_fullscreen(
                        if window.fullscreen_state() == FullscreenType::Desktop {
                            FullscreenType::Off
                        } else {
                            FullscreenType::Desktop
                        }
                    )?;
                },
                _ => {}
            }
        }

        let (window_width, window_height) = canvas.output_size()?;
        let display_scale =
        if window_width as f32 / window_height as f32 > 16.0/9.0 {
            window_height as f32 / HEIGHT as f32
        }
        else {
            window_width as f32 / WIDTH as f32
        };

        canvas.set_scale(display_scale, display_scale)?;
        let view_port_rect = Rect::new(
            ((window_width as f32 - WIDTH as f32 * display_scale) / 2.0 / display_scale) as i32,
            ((window_height as f32 - HEIGHT as f32 * display_scale) / 2.0 / display_scale) as i32,
            WIDTH,
            HEIGHT,
        );
        canvas.set_viewport(view_port_rect);

        for bullet in &mut *player_bullets {
            bullet.tick(delta);
        }
        for bullet in &mut *enemy_bullets {
            bullet.tick(delta);
        }

        player.tick(delta, &event_pump, &mut player_bullets, display_scale, view_port_rect);
        
        for enemy in &mut *enemies {
            enemy.tick(delta, player.pos, &mut enemy_bullets);
        }
        
        player_enemy_collision(&mut enemies, &mut player);
        enemy_bullets_collision(&mut enemies, &mut player_bullets, score, &mut player.health);
        player_bullets_collision(&mut player, &mut enemy_bullets);

        enemy_bullets.retain(|b| !b.dead);
        player_bullets.retain(|b| !b.dead);
        
        if score_timer < 0.0 && player.health > 0 {
            *score += 5;
            score_timer = 1.0;
        }

        if player.health <= 0 && player.explosion_timer < 0.0 {
            return Ok(true);
        }

        canvas.clear();
        canvas.copy(&sky_texture, None, Rect::new(0,0,WIDTH,HEIGHT))?;
        
        draw_text(canvas,
            &pixel_font,
            Vector2::new(
            20.0, HEIGHT as f32 - 64.0),
            false,
            Color::RGB(0, 0, 0),
            &format!("Score: {}", score),
            &texture_creator,
            WIDTH
        )?;

        for bullet in &mut *player_bullets {
            bullet.draw(canvas);
        }
        for bullet in &mut *enemy_bullets {
            bullet.draw(canvas);
        }
        for enemy in &mut *enemies {
            enemy.draw(canvas);
        }
    
        player.draw(canvas);
        canvas.present();
        while 1_000_000_000f32 / ((Instant::now() - last_frame).as_nanos() as f32) >= MAX_FPS {}
    }
}