pub mod gpio {
    pub const PIN_22: u8 = 22;
    pub const PIN_27: u8 = 27;
}

pub mod pwm {
    pub const PIN_12: u8 = 12;
    pub const PIN_13: u8 = 13;

    pub const PERIOD: u64 = 10;
    pub const PULSE: u64 = 2000;
    pub const SERVO_AVG_PULSE: u64 = 1500;
    pub const SERVO_MIN_PULSE: u64 = 1300;
    pub const SERVO_MAX_PULSE: u64 = 1800;
    pub const SERVO_STEP: u64 = 50;

}

pub mod commands {
    pub const MOVE_FORWARD: u8 = 0x01;
    pub const MOVE_BACKWARD: u8 = 0x02;
    pub const TURN_RIGHT: u8 = 0x03;
    pub const TURN_LEFT: u8 = 0x04;
    pub const STOP: u8 = 0x05;
    pub const STOP_INTERRUPT: u8 = 0x06;
}