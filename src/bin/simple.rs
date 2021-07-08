use clap::Clap;
use std::thread::sleep;
use std::time::Duration;

type Result<T> = std::result::Result<T, hopper_face::LedControllerError>;

/// Hopper face controller
#[derive(Clap)]
#[clap(version = "0.0.1", author = "David Weis <dweis7@gmail.com>")]
struct Args {
    /// Serial port to use
    #[clap(short, long)]
    port: String,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();

    let mut controller = hopper_face::LedController::open(&args.port)?;
    for color in hopper_face::ALL_COLORS.iter().cycle() {
        sleep(Duration::from_secs(2));
        controller.send(&hopper_face::ColorPacket::with_color(*color))?;
    }
    Ok(())
}
