extern crate sdl2;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, TextureQuery};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use std::thread::sleep;
use std::time::Duration;

mod game;

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

static FONT_NAME: &str = "assets/fonts/Helvetica.ttf";

static WINDOW_WIDTH: u32 = 800;
static WINDOW_HEIGHT: u32 = 600;

static SQUARE_SIZE: u32 = 160;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut is_high_dpi = false;
    let mut binding = video_subsystem.window("2048", WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut window_builder = binding.position_centered().opengl();

    let (ddpi, _, _) = video_subsystem.display_dpi(0)?;
    if ddpi > 96.0 {
        window_builder = window_builder.allow_highdpi();
        is_high_dpi = true;
    }

    let window = window_builder.build().map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let font_size = if is_high_dpi { 64 } else { 32 };
    let font = ttf_context.load_font(FONT_NAME, font_size).unwrap();
    // End of initialization

    let game = game::Game2048::new();

    let text = "The 2048";
    let (texture, mut rectangle) = create_text(&texture_creator, &font, text, (20, 20));

    rectangle.x = (canvas.viewport().width() as i32 - rectangle.width() as i32) / 2;

    let tiles_assets_path = if is_high_dpi {
        "assets/highdpi/"
    } else {
        "assets/"
    };

    let tile_2_texture = texture_creator.load_texture(format!("{tiles_assets_path}2-tile.png"))?;
    let tile_4_texture = texture_creator.load_texture(format!("{tiles_assets_path}4-tile.png"))?;
    let tile_8_texture = texture_creator.load_texture(format!("{tiles_assets_path}8-tile.png"))?;
    let tile_16_texture =
        texture_creator.load_texture(format!("{tiles_assets_path}16-tile.png"))?;
    let tile_32_texture =
        texture_creator.load_texture(format!("{tiles_assets_path}32-tile.png"))?;
    let tile_64_texture =
        texture_creator.load_texture(format!("{tiles_assets_path}64-tile.png"))?;
    let tile_128_texture =
        texture_creator.load_texture(format!("{tiles_assets_path}128-tile.png"))?;
    let tile_256_texture =
        texture_creator.load_texture(format!("{tiles_assets_path}256-tile.png"))?;
    let tile_512_texture =
        texture_creator.load_texture(format!("{tiles_assets_path}512-tile.png"))?;
    let tile_1024_texture =
        texture_creator.load_texture(format!("{tiles_assets_path}1024-tile.png"))?;
    let tile_2048_texture =
        texture_creator.load_texture(format!("{tiles_assets_path}2048-tile.png"))?;

    let canvas_width = canvas.viewport().width();

    let (tile_x_1, tile_y_1) = calc_rectangle_position(0, 0, canvas_width, is_high_dpi);
    let (tile_x_2, tile_y_2) = calc_rectangle_position(1, 1, canvas_width, is_high_dpi);
    let (tile_x_3, tile_y_3) = calc_rectangle_position(2, 2, canvas_width, is_high_dpi);
    let (tile_x_4, tile_y_4) = calc_rectangle_position(3, 3, canvas_width, is_high_dpi);

    let square_size = if is_high_dpi {
        SQUARE_SIZE
    } else {
        SQUARE_SIZE / 2
    };

    let tile00_rectangle = rect!(tile_x_1, tile_y_1, square_size, square_size);
    let tile01_rectangle = rect!(tile_x_1, tile_y_2, square_size, square_size);
    let tile02_rectangle = rect!(tile_x_1, tile_y_3, square_size, square_size);
    let tile03_rectangle = rect!(tile_x_1, tile_y_4, square_size, square_size);

    let tile10_rectangle = rect!(tile_x_2, tile_y_1, square_size, square_size);
    let tile11_rectangle = rect!(tile_x_2, tile_y_2, square_size, square_size);
    let tile12_rectangle = rect!(tile_x_2, tile_y_3, square_size, square_size);
    let tile13_rectangle = rect!(tile_x_2, tile_y_4, square_size, square_size);

    let tile20_rectangle = rect!(tile_x_3, tile_y_1, square_size, square_size);
    let tile21_rectangle = rect!(tile_x_3, tile_y_2, square_size, square_size);
    let tile22_rectangle = rect!(tile_x_3, tile_y_3, square_size, square_size);
    let tile23_rectangle = rect!(tile_x_3, tile_y_4, square_size, square_size);

    let tile30_rectangle = rect!(tile_x_4, tile_y_1, square_size, square_size);
    let tile31_rectangle = rect!(tile_x_4, tile_y_2, square_size, square_size);
    let tile32_rectangle = rect!(tile_x_4, tile_y_3, square_size, square_size);
    let tile33_rectangle = rect!(tile_x_4, tile_y_4, square_size, square_size);

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
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.clear();

        canvas.copy(&texture, None, rectangle)?;

        canvas.copy(&tile_2_texture, None, tile00_rectangle)?;
        canvas.copy(&tile_4_texture, None, tile01_rectangle)?;
        canvas.copy(&tile_8_texture, None, tile02_rectangle)?;
        canvas.copy(&tile_16_texture, None, tile03_rectangle)?;

        canvas.copy(&tile_32_texture, None, tile10_rectangle)?;
        canvas.copy(&tile_64_texture, None, tile11_rectangle)?;
        canvas.copy(&tile_128_texture, None, tile12_rectangle)?;
        canvas.copy(&tile_256_texture, None, tile13_rectangle)?;

        canvas.copy(&tile_512_texture, None, tile20_rectangle)?;
        canvas.copy(&tile_1024_texture, None, tile21_rectangle)?;
        canvas.copy(&tile_2048_texture, None, tile22_rectangle)?;
        canvas.copy(&tile_2_texture, None, tile23_rectangle)?;

        canvas.copy(&tile_4_texture, None, tile30_rectangle)?;
        canvas.copy(&tile_8_texture, None, tile31_rectangle)?;
        canvas.copy(&tile_16_texture, None, tile32_rectangle)?;
        canvas.copy(&tile_32_texture, None, tile33_rectangle)?;

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

fn calc_rectangle_position(x: u32, y: u32, canvas_width: u32, is_high_dpi: bool) -> (u32, u32) {
    let top_margin = if is_high_dpi { 160 } else { 70 };
    let margin = if is_high_dpi { 16 } else { 10 };
    let square_size = if is_high_dpi {
        SQUARE_SIZE
    } else {
        SQUARE_SIZE / 2
    };
    let field_length = (square_size * 4) + (margin * 3);
    let left_first = (canvas_width - field_length) / 2;

    let x_output = left_first + (x * (square_size + margin));
    let y_output = top_margin + (y * (square_size + margin));

    (x_output, y_output)
}
