use actix_web::{
    get,
    HttpResponse,
    Responder,
    web
};

// use std::sync::Mutex;
// use rppal::gpio::OutputPin;

use std::time::Duration;
// use std::sync::{Arc, Mutex};
// use rppal::gpio::{Gpio, OutputPin};
// use ctor::ctor;
// use rppal::pwm::{Channel, Polarity, Pwm};

// const GPIO_LED_13: u8 = 13;
// const GPIO_LED_19: u8 = 19;
// const GPIO_LED_16: u8 = 16;

// const PERIOD_MS: u64 = 20;
// static mut PWM0_PULSE: Mutex<u64> = Mutex::new(1200);
// const PULSE: u64 = 1200;
// const PULSE_NEUTRAL_US: u64 = 1500;
// const PULSE_MAX_US: u64 = 1800;
// struct GpioPin {
//     pin_13: Option<OutputPin>,
//     pin_16: Option<OutputPin>
// }

// impl GpioPin {
//     fn output_pin (led: u8) -> OutputPin {
//         let gp = match Gpio::new() {
//             Ok(val) => val,
//             Err(e) => panic!("{}", e)
//         };
//         let res = match gp.get(led) {
//             Ok(val) => val,
//             Err(e) => panic!("{}", e)
//         };

//         res.into_output()
//     }

//     fn pwm0_pin (period_ms: u64, pulse_us: u64, 
//                 polarity: Polarity, enable_pin: bool) -> Pwm {

//         return match Pwm::with_period(
//                 Channel::Pwm0,
//                 Duration::from_millis(period_ms),
//                 Duration::from_micros(pulse_us),
//                 polarity,
//                 enable_pin,
//             ) {
//                 Ok(val) => val,
//                 Err(e) => panic!("{}", e)
//             }
//     }
// }

// struct PwmRegulator;

// impl PwmRegulator {
//     fn spedup (pwm: Pwm, pulse: Mutex<u64>) {
//         let pulse = match pulse.lock() {
//             Ok(val) => val,
//             Err(e) => panic!("{}", e)
//         };

//         match pwm.set_pulse_width(Duration::from_micros(*pulse)) {
//             Ok(val) => val,
//             Err(e) => panic!("{}", e)
//         }
//     }
// }

// #[ctor] static PWM0_18: Pwm = GpioPin::pwm0_pin(
//     PERIOD_MS, PULSE, Polarity::Normal, true
// );
//     static mut PIN_16: OutputPin = GpioPin::output_pin(GPIO_LED_16);
// #[ctor] static PIN_19: OutputPin = GpioPin::output_pin(GPIO_LED_19);
// #[ctor] static PIN_13: OutputPin = GpioPin::output_pin(GPIO_LED_13);
// pub struct AppState {
//     pin_13: Mutex<OutputPin>,
//     pin_16: Mutex<OutputPin>,   
// }

use crate::api::AppState;

#[get("/enable")]
async fn enable(data: web::Data<AppState>) -> impl Responder {
    let mut pin_13 = data.pin_13.lock().unwrap();
    let mut pin_16 = data.pin_16.lock().unwrap();
    let mut pin_19 = data.pin_19.lock().unwrap();

    (*pin_13).set_high();
    (*pin_16).set_high();
    (*pin_19).set_low();

    HttpResponse::Ok().json("Pin enabled")
}

#[get("/disable")]
async fn disable(data: web::Data<AppState>) -> impl Responder {
    let mut pin_13 = data.pin_13.lock().unwrap();
    let mut pin_16 = data.pin_16.lock().unwrap();
    (*pin_13).set_low();
    (*pin_16).set_low();

    HttpResponse::Ok().json("Pin disabled")
}

#[get("/setpulse")]
async fn speedup(data: web::Data<AppState>) -> impl Responder {
    let pin_pwm0 = data.pin_pwm0.lock().unwrap();

    let current_pulse = pin_pwm0.pulse_width().unwrap();
    let new_pulse = current_pulse.checked_add(Duration::from_micros(50)).unwrap();
    pin_pwm0.set_pulse_width(new_pulse).unwrap();

    HttpResponse::Ok().json("Pin disabled")
}

#[get("/setperiod")]
async fn setperiod(data: web::Data<AppState>) -> impl Responder {
    let pin_pwm0 = data.pin_pwm0.lock().unwrap();

    let current_period = pin_pwm0.period().unwrap();
    let new_period = current_period.checked_sub(Duration::from_micros(2)).unwrap();
    pin_pwm0.set_pulse_width(new_period).unwrap();

    HttpResponse::Ok().json("Pin period")
}


