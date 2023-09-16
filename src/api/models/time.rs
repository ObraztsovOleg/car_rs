pub mod timer_model {
    use timer;
    use chrono;
    use crate::api::repository::gpio::gpio_repository::set_stop_move;
    use crate::api::models::globals::time;
    use once_cell::sync::Lazy;
    use std::collections::HashMap;
    use std::sync::Mutex;

    pub unsafe fn update_timer_move () {
        let guard_mutex = GUARD.get_mut(&time::GUARD_MOVE).unwrap();
        let mut guard = guard_mutex.lock().unwrap();
        *guard = TIMER_MOVE.schedule_with_delay(chrono::Duration::milliseconds(time::TIMER_MOVE), || { set_stop_move() });
    }

    pub static mut TIMER_MOVE: Lazy<timer::Timer> = Lazy::new(|| timer::Timer::new());

    pub static mut GUARD: Lazy<HashMap<u8, Mutex<timer::Guard>>> = Lazy::new(||
        HashMap::from([
            (time::GUARD_MOVE, Mutex::new(
                unsafe { TIMER_MOVE.schedule_with_delay(chrono::Duration::milliseconds(0), || { set_stop_move() }) }
            ))
        ])
    );
}