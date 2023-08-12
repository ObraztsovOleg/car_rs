pub mod ping;

use std::sync::Mutex;
use rppal::gpio::OutputPin;
use rppal::pwm::Pwm;

pub struct AppState {
    pub pin_13: Mutex<OutputPin>,
    pub pin_16: Mutex<OutputPin>,   
    pub pin_19: Mutex<OutputPin>,
    pub pin_pwm0: Mutex<Pwm> 
}