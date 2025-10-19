use image::RgbaImage;
use sdl3_sys::error::*;
use sdl3_sys::events::*;
use sdl3_sys::init::*;
use sdl3_sys::pixels::*;
use sdl3_sys::rect::*;
use sdl3_sys::render::*;
use sdl3_sys::surface::*;
use sdl3_sys::video::*;
use std::env;
use std::ffi::CStr;
use std::ffi::c_int;
use std::ffi::c_void;
use std::process::ExitCode;
use std::ptr;

const WINDOW_TITLE: &CStr = unsafe {
    CStr::from_bytes_with_nul_unchecked(
        concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"), '\0').as_bytes(),
    )
};

const SDL_INIT_ALL: u32 = 0;

macro_rules! die {
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        return ExitCode::FAILURE;
    }};
}

struct SdlContext {}

impl SdlContext {
    fn new() -> Option<Self> {
        if !unsafe { SDL_Init(SDL_INIT_ALL) } {
            return None;
        }
        Some(SdlContext {})
    }
}

impl Drop for SdlContext {
    fn drop(&mut self) {
        unsafe {
            SDL_Quit();
        }
    }
}

struct Window {
    _sdl: SdlContext,
    raw: *mut SDL_Window,
}

impl Window {
    fn new(w: c_int, h: c_int) -> Option<Self> {
        let _sdl = match SdlContext::new() {
            Some(sdl) => sdl,
            None => return None,
        };

        let raw = unsafe { SDL_CreateWindow(WINDOW_TITLE.as_ptr(), w, h, 0) };

        if raw == ptr::null_mut() {
            return None;
        }

        Some(Window { _sdl, raw })
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyWindow(self.raw);
        }
    }
}

struct Renderer {
    raw: *mut SDL_Renderer,
}

impl Renderer {
    fn new(win: &Window) -> Option<Self> {
        let raw = unsafe { SDL_CreateRenderer(win.raw, ptr::null_mut()) };

        if raw == ptr::null_mut() {
            return None;
        }

        Some(Renderer { raw })
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            SDL_DestroyRenderer(self.raw);
        }
    }
}

struct Args {
    path: String,
    width: c_int,
    height: c_int,
}

impl Args {
    fn parse() -> Option<Self> {
        let mut args = env::args().skip(1);
        let width = args.next()?.parse().ok()?;
        let height = args.next()?.parse().ok()?;
        let path = args.next()?;

        if width < 0 || height < 0 {
            return None;
        }

        Some(Self {
            width,
            height,
            path,
        })
    }
}

fn texture_from_image(r: *mut SDL_Renderer, img: &RgbaImage) -> *mut SDL_Texture {
    let (imgw, imgh) = img.dimensions();
    let mut data = img.as_raw().clone();

    let surface = unsafe {
        SDL_CreateSurfaceFrom(
            imgw as c_int,
            imgh as c_int,
            SDL_PIXELFORMAT_RGBA32,
            data.as_mut_ptr() as *mut c_void,
            imgw as c_int * 4,
        )
    };

    if surface == ptr::null_mut() {
        todo!();
    }

    let texture = unsafe { SDL_CreateTextureFromSurface(r, surface) };

    if texture == ptr::null_mut() {
        todo!();
    }

    unsafe {
        SDL_DestroySurface(surface);
    }

    texture
}

fn render_image(r: *mut SDL_Renderer, img: &RgbaImage) {
    let (mut win_w, mut win_h): (c_int, c_int) = (0, 0);

    if !unsafe { SDL_GetRenderOutputSize(r, &mut win_w, &mut win_h) } {
        todo!();
    }

    let buf = image::imageops::crop_imm(img, 0, 0, win_w as u32, win_h as u32).to_image();

    let sprite = texture_from_image(r, &buf);
    let rect = unsafe {
        SDL_FRect {
            x: 0f32,
            y: 0f32,
            w: (*sprite).w as f32,
            h: (*sprite).h as f32,
        }
    };

    unsafe {
        SDL_RenderTexture(r, sprite, &rect, &rect);
        SDL_DestroyTexture(sprite);
    }
}

fn render_scene(r: *mut SDL_Renderer, img: &RgbaImage) {
    unsafe {
        SDL_SetRenderDrawColor(r, 255, 255, 255, 0);
        SDL_RenderClear(r);
        render_image(r, img);
        SDL_RenderPresent(r);
    }
}

fn handle_event(event: &SDL_Event) -> bool {
    if unsafe { event.r#type } == SDL_EVENT_QUIT.into() {
        return true;
    }

    false
}

struct EventPoller {
    event: SDL_Event,
}

impl Iterator for EventPoller {
    type Item = SDL_Event;

    fn next(&mut self) -> Option<Self::Item> {
        if unsafe { SDL_PollEvent(&mut self.event) } {
            return Some(self.event);
        }

        None
    }
}

fn poll_events() -> EventPoller {
    let event = SDL_Event {
        r#type: SDL_EVENT_QUIT.into(),
    };

    EventPoller { event }
}

fn main() -> ExitCode {
    let Some(args) = Args::parse() else {
        die!("usage: img <width> <height> <path>");
    };

    /* setup */
    let img = match image::open(&args.path) {
        Ok(img) => img.into_rgba8(),
        Err(e) => die!("{}", e),
    };

    let window = match Window::new(args.width, args.height) {
        Some(win) => win,
        None => {
            let msg = unsafe { CStr::from_ptr(SDL_GetError()).to_str().unwrap() };
            die!("{}", msg);
        }
    };

    let renderer = match Renderer::new(&window) {
        Some(r) => r,
        None => {
            let msg = unsafe { CStr::from_ptr(SDL_GetError()).to_str().unwrap() };
            die!("{}", msg);
        }
    };

    /* run */
    'main: loop {
        for event in poll_events() {
            if handle_event(&event) {
                break 'main;
            }
        }
        render_scene(renderer.raw, &img);
    }

    ExitCode::SUCCESS
}
