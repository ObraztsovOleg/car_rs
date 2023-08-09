// mod api;
// mod repository;
// mod models;

// use actix_web::{App, HttpServer};
// use api::ping::enable;

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(move || {
//         App::new()
//             .service(enable)
//     })
//         .bind(("localhost", 5555))?
//         .run()
//         .await
// }

use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

// const GPIO_PWM: u8 = 23;
const GPIO_LED_33: u8 = 33;
const GPIO_LED_35: u8 = 35;
const GPIO_LED_37: u8 = 37;

// const PERIOD_MS: u64 = 20;
// const PULSE_MIN_US: u64 = 1200;
// const PULSE_NEUTRAL_US: u64 = 1500;
// const PULSE_MAX_US: u64 = 1800;

fn main() -> Result<(), Box<dyn Error>> {
    // let mut pin = Gpio::new()?.get(GPIO_PWM)?.into_output();
    let mut pin_33 = Gpio::new()?.get(GPIO_LED_33)?.into_output();
    let mut pin_35 = Gpio::new()?.get(GPIO_LED_35)?.into_output();
    let mut pin_37 = Gpio::new()?.get(GPIO_LED_37)?.into_output();

    pin_35.set_high();
    pin_37.set_high();
    pin_33.set_low();

    // pin.set_pwm(
    //     Duration::from_millis(PERIOD_MS),
    //     Duration::from_micros(PULSE_MAX_US),
    // )?;

    // thread::sleep(Duration::from_millis(500));

    // pin.set_pwm(
    //     Duration::from_millis(PERIOD_MS),
    //     Duration::from_micros(PULSE_MIN_US),
    // )?;

    // thread::sleep(Duration::from_millis(500));

    // for pulse in (PULSE_MIN_US..=PULSE_NEUTRAL_US).step_by(10) {
    //     pin.set_pwm(
    //         Duration::from_millis(PERIOD_MS),
    //         Duration::from_micros(pulse),
    //     )?;
    //     thread::sleep(Duration::from_millis(20));
    // }

    thread::sleep(Duration::from_secs(100000000));

    Ok(())
}