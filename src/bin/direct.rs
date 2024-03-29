use clap::Parser;
use std::thread::sleep;
use std::time::Duration;

type Result<T> = std::result::Result<T, hopper_face::LedControllerError>;

/// Hopper face controller
#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    /// Serial port to use
    #[clap(short, long)]
    port: String,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();
    let mut controller = hopper_face::LedDriver::open(&args.port)?;
    loop {
        for frame in hopper_face::animations::LarsonScanner::new(hopper_face::driver::BRIGHT_PURPLE)
        {
            controller.send(&frame)?;
            sleep(Duration::from_secs_f32(0.03));
        }
    }
}
