use actix_web::{
    get,
    HttpResponse,
    Responder,
    web
};

use crate::AppGpioState;
use crate::api::repository::gpio::gpio_repository::*;

#[get("/enable")]
async fn start(data: web::Data<AppGpioState>) -> impl Responder {
    enable_motor_a(&data);
    // enable_motor_b(&data);

    HttpResponse::Ok().json("Ok")
}

#[get("/reverse")]
async fn reverse(data: web::Data<AppGpioState>) -> impl Responder {
    reverse_motor_a(&data);
    // reverse_motor_b(&data);

    HttpResponse::Ok().json("Ok")
}


#[get("/stop")]
async fn stop(data: web::Data<AppGpioState>) -> impl Responder {
    disable_motor_a(&data);
    // disable_motor_b(&data);

    HttpResponse::Ok().json("Ok")
}

