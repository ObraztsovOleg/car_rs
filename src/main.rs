mod api;
mod repository;
mod models;

use actix_web::{
    App,
    HttpServer,
    web
};
use api::ping::enable;
use crate::api::AppState;
use std::sync::Mutex;


// use std::time::Duration;
use rppal::gpio::{Gpio, OutputPin};


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


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let state = web::Data::new(AppState {
        pin_13: Mutex::new(output_pin(13)),
        pin_16: Mutex::new(output_pin(16))
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(enable)
    })
        .bind(("localhost", 5555))?
        .run()
        .await
}