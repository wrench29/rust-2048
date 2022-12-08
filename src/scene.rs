extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::{Sdl, VideoSubsystem};
use std::thread::sleep;
use std::time::Duration;

pub struct Scene {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    window_size: Option<(u32, u32)>,
}

impl Scene {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        Scene {
            sdl_context,
            video_subsystem,
            window_size: None,
        }
    }
    pub fn init(&mut self, width: u32, height: u32) -> Result<(), String> {
        self.window_size = Some((width, height));

        let window = self
            .video_subsystem
            .window("2048", width, height)
            .position_centered()
            .opengl()
            .allow_highdpi()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        // let text_creator = text::Text::new(&ttf_context, &texture_creator);
        // let (texture, rectangle) = text_creator.create_text("Hello World", Color::RGB(0, 0, 0), (0, 0));

        let mut event_pump = self.sdl_context.event_pump()?;

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

            // canvas.copy(&texture, None, rectangle)?;

            canvas.present();
            sleep(Duration::new(0, 1_000_000_000u32 / 30));
        }

        Ok(())
    }
}
