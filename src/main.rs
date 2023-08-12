mod api;
mod repository;
mod models;

use actix_web::{
    App,
    HttpServer,
    web,
    get,
    HttpResponse,
    Responder
};
// use api::ping::enable;
// use server::gpio_mod;
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

pub struct AppState {
    pin_13: Mutex<OutputPin>,
    pin_16: Mutex<OutputPin>,   
}

#[get("/enable")]
async fn enable(data: web::Data<AppState>) -> impl Responder {
    let mut pin_13 = data.pin_13.lock().unwrap();
    let mut pin_16 = data.pin_16.lock().unwrap();
    (*pin_13).set_high();
    (*pin_16).set_high();

    HttpResponse::Ok().json("Pin enabled")
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