<div align="center">
  <h1 align="center">HC-SR04</h1>

  ![GitHub source size](https://img.shields.io/github/languages/code-size/marcoradocchia/hc-sr04?color=ea6962&logo=github)
  ![GitHub open issues](https://img.shields.io/github/issues-raw/marcoradocchia/hc-sr04?color=%23d8a657&logo=github)
  ![GitHub open pull requests](https://img.shields.io/github/issues-pr-raw/marcoradocchia/hc-sr04?color=%2389b482&logo=github)
  ![GitHub sponsors](https://img.shields.io/github/sponsors/marcoradocchia?color=%23d3869b&logo=github)
  ![Crates.io downloads](https://img.shields.io/crates/d/hc-sr04?label=crates.io%20downloads&color=%23a9b665&logo=rust)
  ![Crates.io version](https://img.shields.io/crates/v/hc-sr04?logo=rust&color=%23d8a657)
  ![GitHub license](https://img.shields.io/github/license/marcoradocchia/hc-sr04?color=%23e78a4e)
</div>

This crate provides a driver for the **HC-SR04**/**HC-SR04P** ultrasonic distance sensor on 
*Raspberry Pi*, using [rppal](https://docs.rs/rppal/0.13.1/rppal/) to access Raspberry Pi's GPIO.

## Examples

Usage examples can be found in the 
[examples](https://github.com/marcoradocchia/hc-sr04/tree/master/examples) folder.

## Measure distance
```rust
use hc_sr04::{HcSr04, Unit};

// Initialize driver.
let mut ultrasonic = HcSr04::new(
    24,          // TRIGGER -> Gpio pin 24
    23,          // ECHO -> Gpio pin 23
    Some(23_f32) // Ambient temperature (if `None` defaults to 20.0C)
).unwrap();

// Perform distance measurement, specifying measuring unit of return value.
match ultrasonic.measure_distance(Unit::Meters).unwrap() {
    Some(dist) => println!("Distance: {.2}m", dist),
    None => println!("Object out of range"),
}
```

## Calibrate measurement

Distance measurement can be calibrated at runtime using the [`HcSr04::calibrate`] method that 
this library exposes, passing the current ambient temperature as `f32`.

```rust
use hc_sr04::{HcSr04, Unit};

// Initialize driver.
let mut ultrasonic = HcSr04::new(24, 23, None).unwrap();

// Calibrate measurement with ambient temperature.
ultrasonic.calibrate(23_f32);

// Perform distance measurement.
match ultrasonic.measure_distance(Unit::Centimeters).unwrap() {
    Some(dist) => println!("Distance: {.1}cm", dist),
    None => println!("Object out of range"),
}
```
