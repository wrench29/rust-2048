// extern crate sdl2;

// macro_rules! rect(
//     ($x:expr, $y:expr, $w:expr, $h:expr) => (
//         Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
//     )
// );

// use sdl2::{
//     pixels::Color,
//     rect::Rect,
//     render::{Texture, TextureCreator, TextureQuery},
//     ttf::Font,
//     ttf::Sdl2TtfContext,
//     video::WindowContext,
// };

// pub struct Text<'a> {
//     font: Font<'a, 'static>,
//     texture_creator: &'a TextureCreator<WindowContext>,
// }

// impl Text<'_> {
//     pub fn new(
//         ttf_context: &Sdl2TtfContext,
//         texture_creator: &TextureCreator<WindowContext>,
//     ) -> Self {
//         let font = ttf_context.load_font("Helvetica.ttf", 64).unwrap();
//         Text {
//             font,
//             texture_creator,
//         }
//     }
//     pub fn create_text(&self, text: &str, color: Color, point: (i32, i32)) -> (Texture, Rect) {
//         let surface = self
//             .font
//             .render(text)
//             .blended(color)
//             .map_err(|e| e.to_string())
//             .unwrap();

//         let texture = self
//             .texture_creator
//             .create_texture_from_surface(surface)
//             .map_err(|e| e.to_string())
//             .unwrap();

//         let TextureQuery { width, height, .. } = texture.query();

//         let rectangle = rect!(point.0, point.1, width, height);
//         (texture, rectangle)
//     }
// }
