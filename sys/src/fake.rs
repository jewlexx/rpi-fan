pub mod gpio {
    use thiserror::Error as AsError;

    #[derive(Debug, Clone, Copy, AsError)]
    pub enum Error {}

    pub struct Gpio;

    impl Gpio {
        pub fn new() -> Result<Gpio, Error> {
            Ok(Gpio)
        }

        pub fn get(&self, _: u8) -> Result<OutputPin, Error> {
            Ok(OutputPin::new())
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct OutputPin {
        is_high: bool,
    }

    impl OutputPin {
        pub fn new() -> Self {
            Self {
                is_high: rand::random(),
            }
        }

        pub fn into_output(&self) -> Self {
            *self
        }

        pub fn is_set_high(&self) -> bool {
            self.is_high
        }

        pub fn set_high(&mut self) {
            self.is_high = true;
        }

        pub fn set_low(&mut self) {
            self.is_high = false;
        }

        pub fn toggle(&mut self) {
            self.is_high = !self.is_high
        }
    }

    impl Default for OutputPin {
        fn default() -> Self {
            Self::new()
        }
    }
}
