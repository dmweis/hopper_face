use clap::Parser;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

type Result<T> = std::result::Result<T, hopper_face::LedControllerError>;

/// Hopper face controller
#[derive(Parser)]
#[clap(author, version)]
struct Args {
    /// Serial port to use
    #[clap(short, long)]
    port: String,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let controller = hopper_face::FaceController::open(&args.port)?;
    controller.larson_scanner(hopper_face::driver::PURPLE)?;
    while running.load(Ordering::SeqCst) {
        sleep(Duration::from_secs_f32(0.1));
    }
    Ok(())
}
