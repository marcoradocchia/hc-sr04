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
//
//! **HC-SR04** ultrasonic distance sensor driver.
//!
//! This crate provides a driver for the **HC-SR04**/**HC-SR04P** ultrasonic distance sensor on
//! *Raspberry Pi*, using [rppal](https://docs.rs/rppal/0.13.1/rppal/) to access Raspberry Pi's
//! GPIO.
//!
//! ## Examples
//!
//! Usage examples can be found in the
//! [examples](https://github.com/marcoradocchia/hc-sr04/tree/master/examples) folder.
//!
//! ## Measure distance
//! ```rust
//! use hc_sr04::{HcSr04, Unit};
//!
//! // Initialize driver.
//! let mut ultrasonic = HcSr04::new(
//!     24,          // TRIGGER -> Gpio pin 24
//!     23,          // ECHO -> Gpio pin 23
//!     Some(23_f32) // Ambient temperature (if `None` defaults to 20.0C)
//! ).unwrap();
//!
//! // Perform distance measurement, specifying measuring unit of return value.
//! match ultrasonic.measure_distance(Unit::Meters).unwrap() {
//!     Some(dist) => println!("Distance: {.2}m", dist),
//!     None => println!("Object out of range"),
//! }
//! ```
//!
//! ## Calibrate measurement
//!
//! Distance measurement can be calibrated at runtime using the [`HcSr04::calibrate`] method that
//! this library exposes, passing the current ambient temperature as `f32`.
//!
//! ```rust
//! use hc_sr04::{HcSr04, Unit};
//!
//! // Initialize driver.
//! let mut ultrasonic = HcSr04::new(24, 23, None).unwrap();
//!
//! // Calibrate measurement with ambient temperature.
//! ultrasonic.calibrate(23_f32);
//!
//! // Perform distance measurement.
//! match ultrasonic.measure_distance(Unit::Centimeters).unwrap() {
//!     Some(dist) => println!("Distance: {.1}cm", dist),
//!     None => println!("Object out of range"),
//! }
//! ```

pub mod error;

use error::Error;
use rppal::gpio::{Gpio, InputPin, Level, OutputPin, Trigger};
use std::{
    thread,
    time::{Duration, Instant},
};

pub type Result<T> = std::result::Result<T, Error>;

/// Measuring unit (defaults to [`Unit::Meters`]).
pub enum Unit {
    Millimeters,
    Centimeters,
    Decimeters,
    Meters,
}

/// **HC-SR04** ultrasonic sensor on *Raspberry Pi*.
///
/// # Fileds
///
/// - `trig`: **TRIGGER** output GPIO pin
/// - `echo`: **ECHO** input GPIO pin
/// - `temp`: ambient **Temperature** measure calibration
/// - `sound_speed`: speed of sound given the ambient **Temperature**
/// - `timeout`: **ECHO** pin polling timeout, considering the maximum measuring range of 4m for
///     the sensor and the speed of sound given the ambient **Temperature**
#[derive(Debug)]
pub struct HcSr04 {
    trig: OutputPin,
    echo: InputPin,
    sound_speed: f32,
    timeout: Duration,
}

impl HcSr04 {
    /// Perform `sound_speed` and `timeout` calculations required to calibrate the sensor,
    /// based on **ambient temperature**.
    fn calibration_calc(temp: f32) -> (f32, Duration) {
        /// Speed of sound at 0C in m/s.
        const SOUND_SPEED_0C: f32 = 331.3;
        /// Increase speed of sound over temperature factor m/[sC].
        const SOUND_SPEED_INC_OVER_TEMP: f32 = 0.606;
        /// Maximum measuring range for HC-SR04 sensor in m.
        const MAX_RANGE: f32 = 4.0;

        // Speed of sound, depending on ambient temperature (if `temp` is `None`, default to 20C).
        let sound_speed = SOUND_SPEED_0C + (SOUND_SPEED_INC_OVER_TEMP * temp);

        // Polling timeout for **ECHO** pin: since max range for HC-SR04 is 4m, it doesn't make
        // sense to wait longer than the time required to the ultrasonic sound wave to cover the
        // max range distance. In other words, if the timeout is reached, the measurement was not
        // successfull or the object is located too far away from the sensor in order to be
        // detected.
        let timeout = Duration::from_secs_f32(MAX_RANGE / sound_speed);

        (sound_speed, timeout)
    }

    /// Initialize HC-SR04 sensor and register GPIO interrupt on `echo` pin for RisingEdge events
    /// in order to poll it for bouncing UltraSonic waves detection.
    ///
    /// # Parameters
    ///
    /// - `trig`: **TRIGGER** output GPIO pin
    /// - `echo`: **ECHO** input GPIO pin
    /// - `temp`: ambient **TEMPERATURE** used for calibration (if `None` defaults to `20.0`)
    pub fn new(trig: u8, echo: u8, temp: Option<f32>) -> Result<Self> {
        let gpio = Gpio::new()?;

        let mut echo = gpio.get(echo)?.into_input_pulldown();
        echo.set_interrupt(Trigger::Both)?;

        let (sound_speed, timeout) = Self::calibration_calc(temp.unwrap_or(20.));

        Ok(Self {
            trig: gpio.get(trig)?.into_output_low(),
            echo,
            sound_speed,
            timeout,
        })
    }

    /// Calibrate the sensor with the given **ambient temperature** (`temp`) expressed as *Celsius
    /// degrees*.
    pub fn calibrate(&mut self, temp: f32) {
        (self.sound_speed, self.timeout) = Self::calibration_calc(temp);
    }

    /// Perform **distance measurement**.
    ///
    /// Returns `Ok` variant if measurement succedes. Inner `Option` value is `None` if no object
    /// is present within maximum measuring range (*4m*); otherwhise, on `Some` variant instead,
    /// contained value represents distance expressed as the specified `unit`
    /// (**unit of measure**).
    pub fn measure_distance(&mut self, unit: Unit) -> Result<Option<f32>> {
        self.trig.set_high();
        thread::sleep(Duration::from_micros(10));
        self.trig.set_low();

        // Wait for the `RisingEdge` by ensuring the resulting level is `Level::High`.
        while self.echo.poll_interrupt(false, None)? != Some(Level::High) {}
        let instant = Instant::now();
        // Wait for the `FallingEdge` by ensuring the resulting level is `Level::Low`.
        if self.echo.poll_interrupt(false, Some(self.timeout))? != Some(Level::Low) {
            // Timeout reached: object out of range (distance > maximum range).
            return Ok(None);
        }

        // Distance in m.
        let distance = (self.sound_speed * instant.elapsed().as_secs_f32()) / 2.;

        Ok(Some(match unit {
            Unit::Millimeters => distance * 1000.,
            Unit::Centimeters => distance * 100.,
            Unit::Decimeters => distance * 10.,
            Unit::Meters => distance,
        }))
    }
}
