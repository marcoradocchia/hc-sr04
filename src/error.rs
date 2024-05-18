// hc-sr04: Raspberry Pi Rust driver for the HC-SR04 ultrasonic distance sensor.
// Copyright (C) 2022 Marco Radocchia
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program. If not, see https://www.gnu.org/licenses/.

use rppal::gpio;
use std::{
    self,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
/// HC-SR04 runtime errors.
pub enum Error {
    /// Occurs on Raspberry Pi GPIO error.
    Gpio(gpio::Error),
    Timeout()
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::Gpio(error) => write!(f, "GPIO error: {}", error),
            Self::Timeout() => write!(f, "Timed out reading GPIO"),
        }
    }
}

impl std::error::Error for Error {}

impl From<gpio::Error> for Error {
    fn from(error: gpio::Error) -> Self {
        Self::Gpio(error)
    }
}
