use std::thread;
use std::time::Duration;

use rppal::pwm::{Channel, Polarity, Pwm};
use actix_web::{
    get,
    HttpResponse,
    Responder
};



#[get("/enable")]
async fn enable() -> impl Responder {
    let pwm = Pwm::with_frequency(Channel::Pwm0, 2.0, 0.25, Polarity::Normal, true).unwrap();
    
    thread::sleep(Duration::from_secs(2));
    pwm.set_frequency(8.0, 0.5).unwrap();
    thread::sleep(Duration::from_secs(3));

    HttpResponse::Ok().json("Pin enabled")
}
