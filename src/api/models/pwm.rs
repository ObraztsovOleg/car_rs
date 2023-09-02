pub mod pwm_model {
    use std::sync::{Mutex, MutexGuard};
    use rppal::pwm::{Channel, Polarity, Pwm};
    use std::time::Duration;
    use once_cell::sync::Lazy;
    use std::collections::HashMap;
    use crate::api::models::globals::pwm;

    pub fn pwm_pin (channel: Channel, period_ms: u64, pulse_us: u64) -> Pwm {
        return match Pwm::with_period (
            channel,
            Duration::from_millis(period_ms),
            Duration::from_micros(pulse_us),
            Polarity::Normal,
            false,
        ) {
            Ok(val) => val,
            Err(e) => panic!("{}", e)
        }
    }

    pub fn update_pulse (pin: MutexGuard<Pwm>, pulse_us: u64) {
        pin.set_pulse_width(Duration::from_micros(pulse_us)).unwrap();
    }

    pub static mut PWM_STATE: Lazy<HashMap<u8, Mutex<Pwm>>> = Lazy::new(||
        HashMap::from([
            (pwm::PIN_12, Mutex::new(pwm_pin(Channel::Pwm0, pwm::PERIOD, pwm::PULSE))),
            (pwm::PIN_13, Mutex::new(pwm_pin(Channel::Pwm1, pwm::PERIOD, pwm::PULSE))),
            (pwm::PIN_18, Mutex::new(pwm_pin(Channel::Pwm0, pwm::PERIOD, pwm::PULSE))),
            (pwm::PIN_19, Mutex::new(pwm_pin(Channel::Pwm1, pwm::PERIOD, pwm::PULSE))),
        ])
    );
}