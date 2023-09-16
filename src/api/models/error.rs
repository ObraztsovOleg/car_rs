pub mod error_model {
    use std::sync::{Mutex, MutexGuard};

    pub fn mutex_guard<T> (mutex: &Mutex<T>)-> MutexGuard<'_, T> {
        return match mutex.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                tracing::info!("Poisoned data in mutex");
                poisoned.into_inner()
            }
        };
    }
}