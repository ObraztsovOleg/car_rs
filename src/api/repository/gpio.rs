pub mod gpio_repository {
    use crate::api::models::pwm::pwm_model::{PWM_STATE, update_pulse};
    use crate::api::models::gpio::gpio_model::GPIO_STATE;
    use crate::api::models::globals::pwm;
    use crate::api::models::globals::gpio;

    pub unsafe fn set_start (forward: bool) {
        let pin_12 = PWM_STATE.get_mut(&pwm::PIN_12).unwrap();
        let pin_22 = GPIO_STATE.get_mut(&gpio::PIN_22).unwrap();
        let pin_27 = GPIO_STATE.get_mut(&gpio::PIN_27).unwrap();

        let mut gpio_22 = pin_22.lock().unwrap();
        let mut gpio_27 = pin_27.lock().unwrap();
        let pwm_12 = pin_12.lock().unwrap();

        if forward {
            gpio_22.set_high();
            gpio_27.set_low();
            pwm_12.enable().unwrap();
            pwm_12.set_duty_cycle(0.5).unwrap();
        } else {
            gpio_27.set_high();
            gpio_22.set_low();
            pwm_12.enable().unwrap();
            pwm_12.set_duty_cycle(0.5).unwrap();
        }
    }

    pub unsafe fn set_stop () {
        let pin_12 = PWM_STATE.get_mut(&pwm::PIN_12).unwrap();
        let pin_22 = GPIO_STATE.get_mut(&gpio::PIN_22).unwrap();
        let pin_27 = GPIO_STATE.get_mut(&gpio::PIN_27).unwrap();

        let mut gpio_22 = pin_22.lock().unwrap();
        let mut gpio_27 = pin_27.lock().unwrap();
        let pwm_12 = pin_12.lock().unwrap();

        pwm_12.disable().unwrap();
        gpio_22.set_low();
        gpio_27.set_low();
    }

    pub unsafe fn set_turnside (left: bool) {
        if left {
            update_pulse(pwm::PIN_13, 1200);
        } else {
            update_pulse(pwm::PIN_13, 1800);
        }
        
    }
}