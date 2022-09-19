use std::io::Read;
use std::{thread::sleep, time::Duration};

fn main() {
    let face_controller = hopper_face::FaceController::open("/dev/hopper_face").unwrap();

    face_controller
        .larson_scanner(hopper_face::driver::PURPLE)
        .unwrap();
    sleep(Duration::from_secs(10));

    face_controller.off().unwrap();
    sleep(Duration::from_secs(3));

    face_controller
        .solid_color(hopper_face::driver::BRIGHT_RED)
        .unwrap();
    sleep(Duration::from_secs(3));

    face_controller
        .run_animation(hopper_face::driver::PURPLE)
        .unwrap();
    sleep(Duration::from_secs(10));

    face_controller.count_down_basic().unwrap();
    sleep(Duration::from_secs(10));

    face_controller
        .breathing(hopper_face::driver::BRIGHT_RED)
        .unwrap();

    println!("Press enter to exit");
    _ = std::io::stdin().read(&mut [0]).unwrap();
}
