pub mod pwm_model {
    use std::sync::Mutex;
    use rppal::pwm::{Channel, Polarity, Pwm};
    use std::time::Duration;

    pub struct AppPwmState {
        pub pin_pwm0: Mutex<Pwm> 
    }

    pub fn pwm0_pin (period_ms: u64, pulse_us: u64) -> Pwm {

        return match Pwm::with_period(
            Channel::Pwm0,
            Duration::from_millis(period_ms),
            Duration::from_micros(pulse_us),
            Polarity::Normal,
            true,
        ) {
            Ok(val) => val,
            Err(e) => panic!("{}", e)
        }
    }
}