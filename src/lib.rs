pub mod animations;
pub mod driver;

use animations::Animation;
use driver::Result;
pub use driver::{ColorPacket, LedControllerError, LedDriver, RGB};
use log::*;
use std::{
    sync::mpsc::{self, sync_channel, SyncSender},
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
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
                    sleep(Duration::from_secs_f32(0.03));
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
        self.sender
            .send(ColorCommand::Animation(Animation::LarsonScanner(color)))
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
