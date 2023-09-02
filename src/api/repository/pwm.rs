// pub mod pwm_repository {
//     use actix_web::web;
//     use crate::AppPwmState;
//     use std::time::Duration;

//     const LEFT_PULSE: u64 = 500;
//     const RIGHT_PULSE: u64 = 500;
//     const CENTRAL_PULSE: u64 = 500;
//     const PWM_STEP: u64 = 500;

//     pub fn increase_pulse(data: &web::Data<AppPwmState>) {
//         let pwm_18 = data.pwm0_18.lock().unwrap();

//         let current_pulse = pwm_18.pulse_width().unwrap();
//         let new_pulse = current_pulse.checked_add(Duration::from_micros(PWM_STEP)).unwrap();
//         pwm_18.set_pulse_width(new_pulse).unwrap();
//     }

//     pub fn decrease_pulse(data: &web::Data<AppPwmState>) {
//         let pwm_18 = data.pwm0_18.lock().unwrap();

//         let current_pulse = pwm_18.pulse_width().unwrap();
//         let new_pulse = current_pulse.checked_sub(Duration::from_micros(PWM_STEP)).unwrap();
//         pwm_18.set_pulse_width(new_pulse).unwrap();
//     }

//     pub fn turn_aside (data: &web::Data<AppPwmState>, side: bool) {
//         todo!("продумать время поворота, если жмешь - поворачивает");
        
//         // let pwm_13 = data.pwm1_13.lock().unwrap();

//         // let pulse = match side { 
//         //     true => RIGHT_PULSE,
//         //     false => LEFT_PULSE
//         // };

//         // pwm_13.set_pulse_width(Duration::from_millis(pulse)).unwrap();
//     }

// }