pub mod gpio_repository {
    use crate::api::models::pwm::pwm_model::{PWM_STATE, update_pulse};
    use crate::api::models::globals::pwm;

    pub unsafe fn enable_move (forward: bool) {
        let pin_12 = PWM_STATE.get_mut(&pwm::PIN_12).unwrap();
        let pin_13 = PWM_STATE.get_mut(&pwm::PIN_13).unwrap();
    
        let pwm_12 = pin_12.lock().unwrap();
        let pwm_13 = pin_13.lock().unwrap();

        if forward {
            pwm_12.enable().unwrap();
            pwm_13.disable().unwrap();
        } else {
            pwm_12.disable().unwrap();
            pwm_13.enable().unwrap();
        }
    }

    pub unsafe fn set_pulse (pulse_us: u64) {
        let pin_12 = PWM_STATE.get_mut(&pwm::PIN_12).unwrap();
        let pin_13 = PWM_STATE.get_mut(&pwm::PIN_13).unwrap();

        update_pulse(pin_12.lock().unwrap(), pulse_us);
        update_pulse(pin_13.lock().unwrap(), pulse_us);
    }

    // pub fn enable_motor_b (data: &web::Data<AppGpioState>) {

    // }

    // pub fn reverse_motor_a (data: &web::Data<AppGpioState>) {

    // }

    // pub fn reverse_motor_b (data: &web::Data<AppGpioState>) {

    // }

    // pub fn disable_motor_a (data: &web::Data<AppGpioState>) {
        
    // }

    // pub fn disable_motor_b (data: &web::Data<AppGpioState>) {

    // }
}