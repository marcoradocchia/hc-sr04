// In this example the HC-SR04 is placed alongside a door to detect door opening/closing events.
//
// Place the sensor so that its action line is parallel to wall housing the the door.
// Configure THRESHOLD_DIST constant so that opening the door corresponds to creating an
// obstacle for the ultrasonic sensor palced at a distance lower than the THRESHOLD_DIST.

use hc_sr04::{HcSr04, Result, Unit};
use std::{thread, time::Duration};

// Threshold distance expressed in meters.
const THRESHOLD_DIST: f32 = 1.2;

fn run() -> Result<()> {
    // TRIGGER on GPIO Pin 24 & ECHO on GPIO Pin 23.
    let mut ultrasonic = HcSr04::new(24, 23, None)?;

    let below_threshold = |ultrasonic: &mut HcSr04| -> Result<bool> {
        Ok(ultrasonic
            .measure_distance(Unit::Meters)?
            .unwrap_or(f32::MAX)
            < THRESHOLD_DIST)
    };

    let mut closed = true;
    loop {
        // If measured distance is lower than THRESHOLD_DIST, door is open.
        if below_threshold(&mut ultrasonic)? == closed {
            closed = !closed;
            match closed {
                true => println!("Door closed!"),
                false => println!("Door opened!"),
            }
        }

        thread::sleep(Duration::from_millis(500));
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
    }
}
