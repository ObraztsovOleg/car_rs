pub mod gpio_model {
    use std::sync::Mutex;
    use rppal::gpio::{Gpio, OutputPin};
    use once_cell::sync::Lazy;
    use std::collections::HashMap;
    use crate::api::models::globals::gpio;

    fn output_pin (led: u8) -> OutputPin {
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

    pub static mut GPIO_STATE: Lazy<HashMap<u8, Mutex<OutputPin>>> = Lazy::new(||
        HashMap::from([
            (gpio::PIN_22, Mutex::new(output_pin(gpio::PIN_22)))
        ])
    );
}
