pub mod animations;
pub mod driver;

use animations::Animation;
use driver::Result;
pub use driver::{ColorPacket, LedControllerError, LedDriver, RGB};
use log::*;
use std::{
    sync::mpsc::{self, sync_channel, SyncSender},
    thread::{spawn, JoinHandle},
};

enum ColorCommand {
    Animation(Animation),
    Exit,
}

pub struct FaceController {
    sender: SyncSender<ColorCommand>,
    thread_handle: Option<JoinHandle<()>>,
}

impl FaceController {
    pub fn open(port_name: &str) -> Result<Self> {
        let (sender, rx) = sync_channel(5);
        let mut driver = LedDriver::open(port_name)?;

        let join_handle = spawn(move || {
            let mut animation = Animation::Off;
            let mut iterator = animation.to_iterator();
            #[allow(clippy::while_let_loop)]
            loop {
                if let Ok(optional_message) = rx.try_recv_optional() {
                    if let Some(message) = optional_message {
                        match message {
                            ColorCommand::Animation(new_animation) => {
                                animation = new_animation;
                                iterator = animation.to_iterator();
                            }
                            ColorCommand::Exit => break,
                        }
                    }
                } else {
                    break;
                }
                if let Some(packet) = iterator.next() {
                    if driver.send(&packet).is_err() {
                        error!("Failed to send message to face controller");
                        break;
                    }
                } else {
                    iterator = animation.to_iterator();
                }
            }
        });
        Ok(FaceController {
            sender,
            thread_handle: Some(join_handle),
        })
    }

    pub fn larson_scanner(&self, color: RGB) -> Result<()> {
        self.animation(Animation::LarsonScanner(color))
    }

    pub fn run_animation(&self, color: RGB) -> Result<()> {
        self.animation(Animation::RunAnimation(color))
    }

    pub fn cycle_all_colors(&self) -> Result<()> {
        self.animation(Animation::CycleAllColors)
    }

    pub fn cycle_bright_colors(&self) -> Result<()> {
        self.animation(Animation::CycleBrightColors)
    }

    pub fn cycle_normal_colors(&self) -> Result<()> {
        self.animation(Animation::CycleNormalColors)
    }

    pub fn count_down_basic(&self) -> Result<()> {
        self.animation(Animation::CountDownBasic)
    }

    pub fn count_down(&self, colors: Vec<RGB>) -> Result<()> {
        self.animation(Animation::CountDown(colors))
    }

    pub fn breathing(&self, color: RGB) -> Result<()> {
        self.animation(Animation::Breathing(color))
    }

    pub fn solid_color(&self, color: RGB) -> Result<()> {
        self.animation(Animation::SolidColor(color))
    }

    pub fn speaking(&self, color: RGB) -> Result<()> {
        self.animation(Animation::Speaking(color))
    }

    pub fn off(&self) -> Result<()> {
        self.animation(Animation::Off)
    }

    fn animation(&self, animation: Animation) -> Result<()> {
        self.sender
            .send(ColorCommand::Animation(animation))
            .map_err(|_| LedControllerError::CommThreadError)?;
        Ok(())
    }
}

impl Drop for FaceController {
    fn drop(&mut self) {
        let _ = self.sender.send(ColorCommand::Exit);
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}

pub trait MpscChannelHelper<T> {
    fn try_recv_optional(&self) -> std::result::Result<Option<T>, mpsc::TryRecvError>;
}

impl<T> MpscChannelHelper<T> for mpsc::Receiver<T> {
    fn try_recv_optional(&self) -> std::result::Result<Option<T>, mpsc::TryRecvError> {
        match self.try_recv() {
            Ok(value) => Ok(Some(value)),
            Err(error) => match error {
                mpsc::TryRecvError::Empty => Ok(None),
                mpsc::TryRecvError::Disconnected => Err(error),
            },
        }
    }
}
