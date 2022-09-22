use rppal::gpio;
use std::{
    self,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub enum Error {
    /// Occurs on Raspberry Pi GPIO error.
    Gpio(gpio::Error),
    /// Occurs when distance measuring fails.
    MeasureFail,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Gpio(error) => write!(f, "GPIO error: {}", error),
            Self::MeasureFail => write!(f, "error occured while measuring distance")
        }
    }
}

impl std::error::Error for Error {}

impl From<gpio::Error> for Error {
    fn from(error: gpio::Error) -> Self {
        Self::Gpio(error)
    }
}
