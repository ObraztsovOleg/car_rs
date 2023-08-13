pub mod gpio_model {
    use std::sync::Mutex;
    use rppal::gpio::{Gpio, OutputPin};

    pub struct AppGpioState {
        pub pin_13: Mutex<OutputPin>,
        pub pin_16: Mutex<OutputPin>,   
        pub pin_19: Mutex<OutputPin>
    }

    pub fn output_pin (led: u8) -> OutputPin {
        let gp = match Gpio::new() {
            Ok(val) => val,
            Err(e) => panic!("{}", e)
        };
        let res = match gp.get(led) {
            Ok(val) => val,
            Err(e) => panic!("{}", e)
        };
    
        res.into_output()
    }
}