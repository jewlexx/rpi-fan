/// Converts an [`Option`] to a [`Result`] with the provided error.
#[macro_export]
macro_rules! opt_to_res {
    [ $opt:expr, $res:expr ] => {
        match $opt {
            Some(guard) => Ok(guard),
            None => Err($res),
        }
    };
}

/// Converts a tries to lock the given mutex and returns a [`FanError`] if the lock fails.
#[macro_export]
macro_rules! lock_inner {
    [ $lock:expr ] => {
        $crate::opt_to_res!($lock.try_lock(), $crate::FanError::LockError)
    };
}

/// Locks the given mutex and blocks the current thread until it can be locked
#[macro_export]
macro_rules! lock_inner_blocking {
    [ $lock:expr ] => {
        $lock.lock()
    };
}
