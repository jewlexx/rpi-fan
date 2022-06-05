/// Converts a try_lock [`Option`] into a [`Result`]
#[macro_export]
macro_rules! lock_inner {
    [ $lock:expr ] => {
        match $lock.try_lock() {
            Some(guard) => Ok(guard),
            None => Err($crate::FanError::LockError),
        }
    };
}

/// Locks the given mutex and blocks the current thread until it can be locked
#[macro_export]
macro_rules! lock_inner_blocking {
    [ $lock:expr ] => {
        $lock.lock()
    };
}
