pub mod gpio {
    pub const PIN_22: u8 = 22;
}

pub mod time {
    pub const GUARD_MOVE: u8 = 0;
    pub const TIMER_MOVE: i64 = 300;
}

pub mod pwm {
    pub const PIN_12: u8 = 12;
    pub const PIN_13: u8 = 13;

    pub const DUTY_CYCLE_MAX: f64 = 0.75;
    pub const DUTY_CYCLE_MIN: f64 = 0.0;
    pub const PERIOD: u64 = 20;
    pub const FREQUENCY: f64 = 500.0;
    pub const SERVO_AVG_PULSE: u64 = 1500;
    pub const SERVO_MIN_PULSE: u64 = 1350;
    pub const SERVO_MAX_PULSE: u64 = 1800;
    pub const SERVO_STEP: u64 = 30;
}

pub mod commands {
    pub const MOVE_FORWARD: u8 = 0x01;
    pub const TURN: u8 = 0x02;
    pub const STOP: u8 = 0x03;
}