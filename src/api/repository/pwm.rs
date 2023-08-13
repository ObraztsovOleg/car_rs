pub mod pwm_repository {
    use actix_web::web;
    use crate::AppPwmState;
    use std::time::Duration;

    pub fn increase_pulse(data: &web::Data<AppPwmState>) {
        let pin_pwm0 = data.pin_pwm0.lock().unwrap();

        let current_pulse = pin_pwm0.pulse_width().unwrap();
        let new_pulse = current_pulse.checked_add(Duration::from_micros(100)).unwrap();
        pin_pwm0.set_pulse_width(new_pulse).unwrap();
    }

    pub fn decrease_pulse(data: &web::Data<AppPwmState>) {
        let pin_pwm0 = data.pin_pwm0.lock().unwrap();

        let current_pulse = pin_pwm0.pulse_width().unwrap();
        let new_pulse = current_pulse.checked_sub(Duration::from_micros(100)).unwrap();
        pin_pwm0.set_pulse_width(new_pulse).unwrap();
    }
}