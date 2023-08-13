use actix_web::{
    get,
    HttpResponse,
    Responder,
    web
};

use crate::AppPwmState;
use crate::api::repository::pwm::pwm_repository::*;

#[get("/speedup")]
async fn speedup(data: web::Data<AppPwmState>) -> impl Responder {
    increase_pulse(&data);

    HttpResponse::Ok().json("Ok")
}

#[get("/speeddown")]
async fn speeddown(data: web::Data<AppPwmState>) -> impl Responder {
    decrease_pulse(&data);

    HttpResponse::Ok().json("Ok")
}
