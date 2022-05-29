cfg_if::cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        pub use rppal::gpio;
        pub mod temp;
    } else {
        mod fake;
        pub use fake::*;
    }
}
