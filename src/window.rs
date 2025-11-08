use crate::app::{Color, Frame, Input, Window};
use sdl3::event::Event;
use sdl3::pixels::PixelFormat;
use sdl3::render::TextureAccess;

const WINDOW_TITLE: &str = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));

pub struct SdlWindow {
    canvas: sdl3::render::WindowCanvas,
    pump: sdl3::EventPump,
}

impl Window for SdlWindow {
    fn init(width: u16, height: u16) -> Self {
        let sdl = sdl3::init().unwrap();

        let video = sdl.video().unwrap();

        let canvas = video
            .window(WINDOW_TITLE, width.into(), height.into())
            .build()
            .unwrap()
            .into_canvas();

        let pump = sdl.event_pump().unwrap();

        Self { canvas, pump }
    }
    fn render(&mut self, frame: Frame) {
        let Color { r, g, b } = frame.background;
        let rect = sdl3::render::FRect {
            w: frame.image.width.into(),
            h: frame.image.height.into(),
            x: 0f32,
            y: 0f32,
        };
        let texture_creator = self.canvas.texture_creator();
        let width: u32 = frame.image.width.into();
        let height: u32 = frame.image.height.into();
        let pitch: usize = (width * 4).try_into().unwrap();

        let mut texture = texture_creator
            .create_texture(PixelFormat::RGBA32, TextureAccess::Streaming, width, height)
            .unwrap();

        let _ = texture.update(None, &frame.image.data, pitch);

        self.canvas.set_draw_color((r, g, b));
        self.canvas.clear();
        let _ = self.canvas.copy(&texture, rect, rect);
        self.canvas.present();
    }
    fn handle_events(&mut self) -> Option<Input> {
        for event in self.pump.poll_iter() {
            if let Event::Quit { .. } = event {
                return Some(Input::Quit);
            }
        }

        None
    }
}
