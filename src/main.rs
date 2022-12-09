extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use std::thread::sleep;
use std::time::Duration;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("2048", 800, 600)
        .position_centered()
        .opengl()
        .allow_highdpi()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let text = "Hello Jabroni";
    let color = Color::RGB(0, 0, 0);
    let point = (10, 10);

    let font = ttf_context.load_font("Helvetica.ttf", 64).unwrap();
    let surface = font
        .render(text)
        .blended(color)
        .map_err(|e| e.to_string())
        .unwrap();

    let texture = texture_creator
        .create_texture_from_surface(surface)
        .map_err(|e| e.to_string())
        .unwrap();

    let TextureQuery { width, height, .. } = texture.query();

    let rectangle = rect!(point.0, point.1, width, height);

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(211, 211, 211));
        canvas.clear();

        canvas.copy(&texture, None, rectangle)?;

        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
