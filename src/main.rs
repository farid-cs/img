use app::Application;
use std::env;
use std::process::ExitCode;
use window::SdlWindow;

mod app;
mod window;

struct Args {
    path: String,
    width: u16,
    height: u16,
}

impl Args {
    fn parse() -> Option<Self> {
        let mut args = env::args().skip(1);
        let width = args.next()?.parse().ok()?;
        let height = args.next()?.parse().ok()?;
        let path = args.next()?;

        Some(Self {
            path,
            width,
            height,
        })
    }
}

fn main() -> ExitCode {
    let Some(args) = Args::parse() else {
        eprintln!("usage: img <width> <height> <path>");
        return ExitCode::FAILURE;
    };

    let mut app = Application::<SdlWindow>::init(args);

    app.run();

    ExitCode::SUCCESS
}
