pub mod gpio {
    pub const PIN_22: u8 = 22;
    pub const PIN_27: u8 = 27;
}

pub mod pwm {
    pub const PIN_18: u8 = 18;
    pub const PIN_19: u8 = 19;

    pub const PERIOD: u64 = 10;
    pub const PULSE: u64 = 1000;
    pub const SERVO_PULSE: u64 = 1500;
}

pub mod commands {
    pub const MOVE_FORWARD: u8 = 0x01;
    pub const MOVE_BACKWARD: u8 = 0x02;
    pub const TURN_RIGHT: u8 = 0x03;
    pub const TURN_LEFT: u8 = 0x04;
    pub const STOP: u8 = 0x05;
}