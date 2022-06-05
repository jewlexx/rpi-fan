/// Converts a try_lock [`Option`] into a [`Result`]
#[macro_export]
macro_rules! lock_inner {
    ($lock:expr) => {
        match $lock.try_lock() {
            Some(guard) => Ok(guard),
            None => Err($crate::FanError::LockError),
        }
    };
}
