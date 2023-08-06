use rust_gpiozero::*;
use actix_web::{
    get,
    HttpResponse,
    Responder
};
use ctor::ctor;
use std::sync::{Arc, Mutex};

#[ctor]
static LED_16: Arc<Mutex<rust_gpiozero::LED>> = Arc::new(Mutex::new(LED::new(16)));


#[get("/enable")]
async fn enable() -> impl Responder {
    let led_mutex = Arc::clone(&LED_16);
    let led = led_mutex.lock().unwrap();
    led.on();
    HttpResponse::Ok().json("Pin enabled")
}

#[get("/disable")]
async fn disable() -> impl Responder {
    let led_mutex = Arc::clone(&LED_16);
    let led = led_mutex.lock().unwrap();
    led.off();
    HttpResponse::Ok().json("Pin disabled")
}