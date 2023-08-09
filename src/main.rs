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
const GPIO_LED_13: u8 = 13;
const GPIO_LED_19: u8 = 19;
const GPIO_LED_16: u8 = 16;

// const PERIOD_MS: u64 = 20;
// const PULSE_MIN_US: u64 = 1200;
// const PULSE_NEUTRAL_US: u64 = 1500;
// const PULSE_MAX_US: u64 = 1800;

fn main() -> Result<(), Box<dyn Error>> {
    // let mut pin = Gpio::new()?.get(GPIO_PWM)?.into_output();
    let mut pin_16 = Gpio::new()?.get(GPIO_LED_16)?.into_output();
    let mut pin_19 = Gpio::new()?.get(GPIO_LED_19)?.into_output();
    let mut pin_13 = Gpio::new()?.get(GPIO_LED_13)?.into_output();

    pin_16.set_high();
    pin_19.set_high();
    pin_13.set_low();

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