use rust_gpiozero::*;
use actix_web::{
    get,
    HttpResponse,
    Responder
};
use ctor::ctor;
// use std::sync::{Arc, Mutex};

#[ctor]
static LED_16: LED = LED::new(16);


#[get("/enable")]
async fn enable() -> impl Responder {
    LED_16.on();
    HttpResponse::Ok().json("Pin enabled")
}

#[get("/disable")]
async fn disable() -> impl Responder {
    LED_16.off();
    HttpResponse::Ok().json("Pin disabled")
}