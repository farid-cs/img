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
}

impl<W: Window> Application<W> {
    pub fn init(args: Args) -> Self {
        let image: Image = image::open(args.path).unwrap().into();
        let width = args.width;
        let height = args.height;
        let window = W::init(width, height);

        Self {
            image,
            window,
            width,
            height,
        }
    }
    fn present(&self) -> Frame {
        let max_w = self.width.into();
        let max_h = self.height.into();
        let cropped: Image = imageops::crop_imm(&self.image, 0, 0, max_w, max_h).to_image();
        let image: RawImage = cropped.into();

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
