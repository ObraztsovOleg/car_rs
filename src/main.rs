mod api;
mod repository;
mod models;

use actix_web::{
    App,
    HttpServer,
    web
};
use api::ping::{enable, disable};
use crate::api::AppState;
use std::sync::Mutex;
use rppal::pwm::{Channel, Polarity, Pwm};
use std::time::Duration;
use rppal::gpio::{Gpio, OutputPin};

const PERIOD_MS: u64 = 20;
static mut PULSE_US: u64 = 1200;
// const PULSE_NEUTRAL_US: u64 = 1500;
// const PULSE_MAX_US: u64 = 1800;

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

fn pwm0_pin (period_ms: u64, pulse_us: u64, 
            polarity: Polarity, enable_pin: bool) -> Pwm {

    return match Pwm::with_period(
            Channel::Pwm0,
            Duration::from_millis(period_ms),
            Duration::from_micros(pulse_us),
            polarity,
            enable_pin,
        ) {
            Ok(val) => val,
            Err(e) => panic!("{}", e)
        }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let state = web::Data::new(AppState {
        pin_13: Mutex::new(output_pin(13)),
        pin_16: Mutex::new(output_pin(16)),
        pin_19: Mutex::new(output_pin(19)),
        pin_pwm0: Mutex::new(pwm0_pin(PERIOD_MS, unsafe { PULSE_US }, Polarity::Normal, true))

    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(enable)
            .service(disable)
    })
        .bind(("localhost", 5555))?
        .run()
        .await
}