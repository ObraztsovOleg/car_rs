pub mod gpio_repository {
    use crate::api::models::pwm::pwm_model::{PWM_STATE, update_pulse};
    use crate::api::models::gpio::gpio_model::GPIO_STATE;
    use crate::api::models::globals::pwm;
    use crate::api::models::globals::gpio;

    pub unsafe fn set_start (forward: bool) {
        let pin_18 = PWM_STATE.get_mut(&pwm::PIN_18).unwrap();
        let pin_22 = GPIO_STATE.get_mut(&gpio::PIN_22).unwrap();
        let pin_27 = GPIO_STATE.get_mut(&gpio::PIN_27).unwrap();

        let mut gpio_22 = pin_22.lock().unwrap();
        let mut gpio_27 = pin_27.lock().unwrap();
        let pwm_18 = pin_18.lock().unwrap();

        if forward {
            gpio_22.set_high();
            gpio_27.set_low();
            pwm_18.enable().unwrap();
            pwm_18.set_duty_cycle(0.5).unwrap();
        } else {
            gpio_27.set_high();
            gpio_22.set_low();
            pwm_18.enable().unwrap();
            pwm_18.set_duty_cycle(0.5).unwrap();
        }
    }

    pub unsafe fn set_stop () {
        let pin_18 = PWM_STATE.get_mut(&pwm::PIN_18).unwrap();
        let pin_22 = GPIO_STATE.get_mut(&gpio::PIN_22).unwrap();
        let pin_27 = GPIO_STATE.get_mut(&gpio::PIN_27).unwrap();

        let mut gpio_22 = pin_22.lock().unwrap();
        let mut gpio_27 = pin_27.lock().unwrap();
        let pwm_18 = pin_18.lock().unwrap();

        pwm_18.disable().unwrap();
        gpio_22.set_low();
        gpio_27.set_low();
    }

    pub unsafe fn set_turnside (left: bool) {
        if left {
            update_pulse(pwm::PIN_19, 1000);
        } else {
            update_pulse(pwm::PIN_19, 2000);
        }
        
    }
}