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
    let controller = hopper_face::FaceController::open(&args.port)?;
    controller.larson_scanner(hopper_face::driver::PURPLE)?;
    sleep(Duration::from_secs(5));
    Ok(())
}
