pub mod gpio_repository {
    use actix_web::web;
    use crate::AppGpioState;

    pub fn enable_motor_a (data: &web::Data<AppGpioState>) {
        let mut pin_13 = data.pin_13.lock().unwrap();
        let mut pin_16 = data.pin_16.lock().unwrap();
        let mut pin_19 = data.pin_19.lock().unwrap();
    
        (*pin_13).set_high();
        (*pin_16).set_high();
        (*pin_19).set_low();    
    }

    // pub fn enable_motor_b (data: &web::Data<AppGpioState>) {

    // }

    pub fn reverse_motor_a (data: &web::Data<AppGpioState>) {
        let mut pin_13 = data.pin_13.lock().unwrap();
        let mut pin_16 = data.pin_16.lock().unwrap();
        let mut pin_19 = data.pin_19.lock().unwrap();


        (*pin_13).set_low();
        (*pin_16).set_high();
        (*pin_19).set_high();
    }

    // pub fn reverse_motor_b (data: &web::Data<AppGpioState>) {

    // }

    pub fn disable_motor_a (data: &web::Data<AppGpioState>) {
        let mut pin_13 = data.pin_13.lock().unwrap();
        let mut pin_16 = data.pin_16.lock().unwrap();
        let mut pin_19 = data.pin_19.lock().unwrap();

        (*pin_13).set_low();
        (*pin_16).set_low();
        (*pin_19).set_low();
    }

    // pub fn disable_motor_b (data: &web::Data<AppGpioState>) {

    // }
}