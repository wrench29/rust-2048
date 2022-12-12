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
use std::collections::HashMap;
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

    let mut game = game::Game2048::new();
    game.start();

    let text = "The 2048";
    let (texture, mut rectangle) = create_text(&texture_creator, &font, text, (20, 20));

    rectangle.x = (canvas.viewport().width() as i32 - rectangle.width() as i32) / 2;

    let tiles_assets_path = if is_high_dpi {
        "assets/highdpi/"
    } else {
        "assets/"
    };

    let mut tiles_textures = HashMap::<i32, Texture>::new();

    tiles_textures.insert(
        2,
        texture_creator.load_texture(format!("{tiles_assets_path}2-tile.png"))?,
    );
    tiles_textures.insert(
        4,
        texture_creator.load_texture(format!("{tiles_assets_path}4-tile.png"))?,
    );
    tiles_textures.insert(
        8,
        texture_creator.load_texture(format!("{tiles_assets_path}8-tile.png"))?,
    );
    tiles_textures.insert(
        16,
        texture_creator.load_texture(format!("{tiles_assets_path}16-tile.png"))?,
    );
    tiles_textures.insert(
        32,
        texture_creator.load_texture(format!("{tiles_assets_path}32-tile.png"))?,
    );
    tiles_textures.insert(
        64,
        texture_creator.load_texture(format!("{tiles_assets_path}64-tile.png"))?,
    );
    tiles_textures.insert(
        128,
        texture_creator.load_texture(format!("{tiles_assets_path}128-tile.png"))?,
    );
    tiles_textures.insert(
        256,
        texture_creator.load_texture(format!("{tiles_assets_path}256-tile.png"))?,
    );
    tiles_textures.insert(
        512,
        texture_creator.load_texture(format!("{tiles_assets_path}512-tile.png"))?,
    );
    tiles_textures.insert(
        1024,
        texture_creator.load_texture(format!("{tiles_assets_path}1024-tile.png"))?,
    );
    tiles_textures.insert(
        2048,
        texture_creator.load_texture(format!("{tiles_assets_path}2048-tile.png"))?,
    );

    let tiles_background_texture =
        texture_creator.load_texture(format!("{tiles_assets_path}tiles_background.png"))?;

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

    let mut tiles_rectangles: Vec<Vec<Rect>> = Vec::<Vec<Rect>>::new();

    for _ in 0..4 {
        tiles_rectangles.push(Vec::<Rect>::new());
    }

    tiles_rectangles[0].push(rect!(tile_x_1, tile_y_1, square_size, square_size));
    tiles_rectangles[0].push(rect!(tile_x_1, tile_y_2, square_size, square_size));
    tiles_rectangles[0].push(rect!(tile_x_1, tile_y_3, square_size, square_size));
    tiles_rectangles[0].push(rect!(tile_x_1, tile_y_4, square_size, square_size));

    tiles_rectangles[1].push(rect!(tile_x_2, tile_y_1, square_size, square_size));
    tiles_rectangles[1].push(rect!(tile_x_2, tile_y_2, square_size, square_size));
    tiles_rectangles[1].push(rect!(tile_x_2, tile_y_3, square_size, square_size));
    tiles_rectangles[1].push(rect!(tile_x_2, tile_y_4, square_size, square_size));

    tiles_rectangles[2].push(rect!(tile_x_3, tile_y_1, square_size, square_size));
    tiles_rectangles[2].push(rect!(tile_x_3, tile_y_2, square_size, square_size));
    tiles_rectangles[2].push(rect!(tile_x_3, tile_y_3, square_size, square_size));
    tiles_rectangles[2].push(rect!(tile_x_3, tile_y_4, square_size, square_size));

    tiles_rectangles[3].push(rect!(tile_x_4, tile_y_1, square_size, square_size));
    tiles_rectangles[3].push(rect!(tile_x_4, tile_y_2, square_size, square_size));
    tiles_rectangles[3].push(rect!(tile_x_4, tile_y_3, square_size, square_size));
    tiles_rectangles[3].push(rect!(tile_x_4, tile_y_4, square_size, square_size));

    let tiles_background_rectangle = calc_tiles_background_rectangle(canvas_width, is_high_dpi);

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

        canvas.copy(&tiles_background_texture, None, tiles_background_rectangle)?;

        for x in 0..4 {
            for y in 0..4 {
                let val = game.get_cell(x, y);
                if val == 0 {
                    continue;
                }
                canvas.copy(
                    tiles_textures.get(&val).unwrap(),
                    None,
                    tiles_rectangles[x][y],
                )?;
            }
        }

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

fn calc_tiles_background_rectangle(canvas_width: u32, is_high_dpi: bool) -> Rect {
    let tiles_background_square_size = if is_high_dpi { 720 } else { 370 };
    let square_size = if is_high_dpi {
        SQUARE_SIZE
    } else {
        SQUARE_SIZE / 2
    };

    let margin = if is_high_dpi { 16 } else { 10 };
    let field_length = (square_size * 4) + (margin * 3);
    let top_margin = (if is_high_dpi { 160 } else { 70 }) - margin;
    let left_first = ((canvas_width - field_length) / 2) - margin;

    rect!(
        left_first,
        top_margin,
        tiles_background_square_size,
        tiles_background_square_size
    )
}
