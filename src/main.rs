extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator, TextureQuery};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use std::thread::sleep;
use std::time::Duration;

mod game;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

static FONT_NAME: &str = "Helvetica.ttf";

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

static SQUARE_SIZE: u32 = 64;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("2048", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .allow_highdpi()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let font = ttf_context.load_font(FONT_NAME, 64).unwrap();
    // End of initialization

    let game = game::Game2048::new();

    let text = "The 2048";
    let (texture, mut rectangle) = create_text(&texture_creator, &font, text, (20, 20));

    let start_x = WINDOW_WIDTH - rectangle.width() / 2;
    rectangle.x = start_x as i32;

    // let texture = texture_creator.load_texture("square.png")?;
    // let rectangle = rect!(30, 30, 64, 64);

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::MouseButtonDown {
                    x,
                    y,
                    mouse_btn: MouseButton::Left,
                    ..
                } => {}
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(251, 206, 177));
        canvas.clear();

        canvas.copy(&texture, None, rectangle)?;

        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn create_text<'tc>(
    texture_creator: &'tc TextureCreator<WindowContext>,
    font: &Font,
    text: &str,
    point: (i32, i32),
) -> (Texture<'tc>, Rect) {
    let color = Color::RGB(0, 0, 0);

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

    (texture, rectangle)
}
