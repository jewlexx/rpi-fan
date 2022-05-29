cfg_if::cfg_if! {
    if #[cfg(target = "aarch64-unknown-linux-musl")] {
        pub use rppal::gpio;
        pub mod temp;
    } else {
        mod fake;
        pub use fake::*;
    }
}
