// use std::thread;
// use std::time::Duration;
// use ctor::ctor;
// use rppal::pwm::{Channel, Polarity, Pwm};
// use actix_web::{
//     get,
//     HttpResponse,
//     Responder
// };

// #[ctor]
// static PWM: Pwm = Pwm::with_frequency(Channel::Pwm0, 2.0, 0.25, Polarity::Normal, true).unwrap();

// #[get("/enable")]
// async fn enable() -> impl Responder {
//     thread::sleep(Duration::from_secs(2));
//     PWM.set_frequency(8.0, 0.5).unwrap();
//     thread::sleep(Duration::from_secs(3));

//     HttpResponse::Ok().json("Pin enabled")
// }
