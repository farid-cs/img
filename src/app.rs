use crate::Args;
use image::RgbaImage as Image;
use image::imageops;

const BACKGROUND_COLOR: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct RawImage {
    pub data: Vec<u8>,
    pub width: u16,
    pub height: u16,
}

impl From<Image> for RawImage {
    fn from(value: Image) -> Self {
        let (width, height) = value.dimensions();
        let data = value.into_raw();

        Self {
            data,
            width: width.try_into().expect("too big image"),
            height: height.try_into().expect("too big image"),
        }
    }
}

#[derive(PartialEq)]
pub enum Input {
    Quit,
}

pub struct Frame {
    pub background: Color,
    pub image: RawImage,
}

pub trait Window {
    fn init(w: u16, h: u16) -> Self;
    fn render(&mut self, frame: Frame);
    fn handle_events(&mut self) -> Option<Input>;
}

pub struct Application<Window> {
    image: Image,
    window: Window,
    width: u16,
    height: u16,
    ratio: f32,
}

impl<W: Window> Application<W> {
    pub fn init(args: Args) -> Self {
        let image: Image = image::open(args.path).unwrap().into();
        let width = args.width;
        let height = args.height;
        let window = W::init(width, height);
        let ratio_w = image.width() as f32 / width as f32;
        let ratio_h = image.height() as f32 / height as f32;
        let mut ratio = 1.0;

        if ratio_w > 1.0 || ratio_h > 1.0 {
            if ratio_w > ratio_h {
                ratio = ratio_w;
            } else {
                ratio = ratio_h;
            }
        }

        Self {
            image,
            window,
            width,
            height,
            ratio,
        }
    }
    fn present(&self) -> Frame {
        let new_w = (self.image.width() as f32 / self.ratio) as u32;
        let new_h = (self.image.height() as f32 / self.ratio) as u32;

        let resized: Image =
            imageops::resize(&self.image, new_w, new_h, imageops::FilterType::Nearest);
        let image: RawImage = resized.into();

        Frame {
            background: BACKGROUND_COLOR,
            image,
        }
    }
    pub fn run(&mut self) {
        loop {
            if let Some(input) = self.window.handle_events()
                && input == Input::Quit
            {
                break;
            }

            let frame = self.present();
            self.window.render(frame);
        }
    }
}
