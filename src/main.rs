mod api;
use crate::api::models::gpio::gpio_model::*;
use crate::api::models::pwm::pwm_model::*;

use actix_web::{
    App, HttpServer, web
};
use api::controller::gpio::{
    start, reverse, stop
};
use api::controller::pwm::{
    speeddown, speedup
};
use std::sync::Mutex;

const PERIOD_MS: u64 = 20;
const PULSE_US: u64 = 1000;
const PIN_13: u8 = 13;
const PIN_16: u8 = 16;
const PIN_19: u8 = 19;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let gpio_state = web::Data::new(AppGpioState {
        pin_13: Mutex::new(output_pin(PIN_13)),
        pin_16: Mutex::new(output_pin(PIN_16)),
        pin_19: Mutex::new(output_pin(PIN_19))
    });

    let pwm_state = web::Data::new(AppPwmState {
        pin_pwm0: Mutex::new(pwm0_pin(PERIOD_MS, PULSE_US))
    });

    HttpServer::new(move || {
        App::new()
            .app_data(gpio_state.clone())
            .app_data(pwm_state.clone())
            .service(start)
            .service(speeddown)
            .service(speedup)
            .service(reverse)
            .service(stop)
    })
        .bind(("localhost", 5555))?
        .run()
        .await
}