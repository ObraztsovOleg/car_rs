use actix_web::{
    get,
    HttpResponse,
    Responder,
    web
};

use std::time::Duration;
use crate::api::AppState;

#[get("/enable")]
async fn enable(data: web::Data<AppState>) -> impl Responder {
    let mut pin_13 = data.pin_13.lock().unwrap();
    let mut pin_16 = data.pin_16.lock().unwrap();
    let mut pin_19 = data.pin_19.lock().unwrap();

    (*pin_13).set_high();
    (*pin_16).set_high();
    (*pin_19).set_low();

    HttpResponse::Ok().json("Ok")
}

#[get("/speedup")]
async fn speedup(data: web::Data<AppState>) -> impl Responder {
    let pin_pwm0 = data.pin_pwm0.lock().unwrap();

    let current_pulse = pin_pwm0.pulse_width().unwrap();
    let new_pulse = current_pulse.checked_add(Duration::from_micros(100)).unwrap();
    pin_pwm0.set_pulse_width(new_pulse).unwrap();

    HttpResponse::Ok().json("Ok")
}

#[get("/speeddown")]
async fn speeddown(data: web::Data<AppState>) -> impl Responder {
    let pin_pwm0 = data.pin_pwm0.lock().unwrap();

    let current_pulse = pin_pwm0.pulse_width().unwrap();
    let new_pulse = current_pulse.checked_sub(Duration::from_micros(100)).unwrap();
    pin_pwm0.set_pulse_width(new_pulse).unwrap();

    HttpResponse::Ok().json("Ok")
}

#[get("/reverse")]
async fn reverse(data: web::Data<AppState>) -> impl Responder {
    let mut pin_13 = data.pin_13.lock().unwrap();
    let mut pin_16 = data.pin_16.lock().unwrap();
    let mut pin_19 = data.pin_19.lock().unwrap();

    (*pin_13).set_low();
    (*pin_16).set_high();
    (*pin_19).set_high();

    HttpResponse::Ok().json("Ok")
}


#[get("/stop")]
async fn stop(data: web::Data<AppState>) -> impl Responder {
    let mut pin_13 = data.pin_13.lock().unwrap();
    let mut pin_16 = data.pin_16.lock().unwrap();
    let mut pin_19 = data.pin_19.lock().unwrap();

    (*pin_13).set_low();
    (*pin_16).set_low();
    (*pin_19).set_low();

    HttpResponse::Ok().json("Ok")
}

