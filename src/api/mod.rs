pub mod ping;

use std::sync::Mutex;
use rppal::gpio::OutputPin;
pub struct AppState {
    pub pin_13: Mutex<OutputPin>,
    pub pin_16: Mutex<OutputPin>,   
}