use rust_gpiozero::*;
use actix_web::{
    get,
    HttpResponse,
    Responder
};

#[get("/enable")]
async fn enable() -> impl Responder {
    let led_16 = LED::new(16);

    led_16.on();
    HttpResponse::Ok().json("Pin enabled")
}

#[get("/disable")]
async fn disable() -> impl Responder {
    let led_16 = LED::new(16);

    led_16.off();
    HttpResponse::Ok().json("Pin disabled")
}