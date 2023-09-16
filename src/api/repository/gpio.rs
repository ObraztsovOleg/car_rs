pub mod gpio_repository {
    use crate::api::models::pwm::pwm_model::PWM_STATE;
    use crate::api::models::gpio::gpio_model::GPIO_STATE;
    use crate::api::models::globals::pwm;
    use crate::api::models::globals::gpio;
    use std::time::Duration;
    use std::thread;
    use std::sync::Arc;
    use crate::api::models::time::timer_model::{update_timer_move, update_timer_turn};

    pub unsafe fn set_start (forward: bool) {
        let pin_12 = PWM_STATE.get_mut(&pwm::PIN_12).unwrap();
        let pin_22 = GPIO_STATE.get_mut(&gpio::PIN_22).unwrap();

        let mut gpio_22 = pin_22.lock().unwrap();
        let pwm_12 = pin_12.lock().unwrap();

        if !forward {
            gpio_22.set_low();
            pwm_12.enable().unwrap();
            pwm_12.set_duty_cycle(0.75).unwrap();
        } else {
            gpio_22.set_high();
            pwm_12.enable().unwrap();
            pwm_12.set_duty_cycle(0.0).unwrap();
        }

        update_timer_move();
    }

    pub unsafe fn set_stop_move () {
        let pin_12 = PWM_STATE.get_mut(&pwm::PIN_12).unwrap();
        let pin_22 = GPIO_STATE.get_mut(&gpio::PIN_22).unwrap();

        let mut gpio_22 = pin_22.lock().unwrap();
        let pwm_12 = pin_12.lock().unwrap();

        pwm_12.disable().unwrap();
        gpio_22.set_low();
    }

    pub unsafe fn set_stop_turn() {
        let pin = PWM_STATE.get_mut(&pwm::PIN_13).unwrap();
        let pwm_pin = pin.lock().unwrap();

        pwm_pin.set_pulse_width(Duration::from_micros(pwm::SERVO_AVG_PULSE)).unwrap();
    }

    pub unsafe fn set_turnside (array: Arc<Vec<u8>>) {
        let left = array[1] == 0;
        let position = array[2];
        let pin = PWM_STATE.get_mut(&pwm::PIN_13).unwrap();
        let pwm_pin = pin.lock().unwrap();
        
        if !left {
            let pulse = ((position as u64 - 0) * (pwm::SERVO_MAX_PULSE - pwm::SERVO_AVG_PULSE) / 100) + pwm::SERVO_AVG_PULSE;
            pwm_pin.set_pulse_width(Duration::from_micros(pulse)).unwrap();
        } else {
            let pulse = ((position as u64 - 0) * (pwm::SERVO_AVG_PULSE - pwm::SERVO_MIN_PULSE) / 100) + pwm::SERVO_MIN_PULSE;
            pwm_pin.set_pulse_width(Duration::from_micros(pulse as u64)).unwrap();

        }

        update_timer_turn();
    }

    pub unsafe fn set_execution () {
        set_stop_turn();
        set_stop_move();
    }
}