pub mod gpio {
    pub const PIN_16: u8 = 16;
    pub const PIN_17: u8 = 17;
    pub const PIN_22: u8 = 22;
    pub const PIN_27: u8 = 27;
}

pub mod pwm {
    pub const PIN_12: u8 = 12;
    pub const PIN_13: u8 = 13;
    pub const PIN_18: u8 = 18;
    pub const PIN_19: u8 = 19;

    pub const FREQUENCY: f64 = 2.0;
    pub const DUTY_CYCLE: f64 = 0.25;
}

pub mod commands {
    pub const ROTATE: u8 = 0x01;
    pub const MOVE_FORWARD: u8 = 0x02;
    pub const MOVE_BACKWARD: u8 = 0x03;
}