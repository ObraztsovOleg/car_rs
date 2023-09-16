pub mod timer_model {
    use timer;
    use chrono;
    use crate::api::repository::gpio::gpio_repository::set_stop;
    use crate::api::models::globals::time;
    use once_cell::sync::Lazy;
    use std::collections::HashMap;
    use std::sync::Mutex;

    pub unsafe fn update_timer () {
        let guard_mutex = GUARD.get_mut(&time::GUARD).unwrap();
        let mut guard = guard_mutex.lock().unwrap();
        *guard = TIMER.schedule_with_delay(chrono::Duration::milliseconds(time::TIMER), || { set_stop() });
    }
    pub static mut TIMER: Lazy<timer::Timer> = Lazy::new(|| timer::Timer::new());

    pub static mut GUARD: Lazy<HashMap<u8, Mutex<timer::Guard>>> = Lazy::new(||
        HashMap::from([
            (time::GUARD, Mutex::new(
                unsafe { TIMER.schedule_with_delay(chrono::Duration::milliseconds(0), || { set_stop() }) }
            ))
        ])
    );
}