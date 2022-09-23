use hc_sr04::{HcSr04, Result, Unit};
use std::{thread, time::Duration};

fn run() -> Result<()> {
    // TRIGGER on GPIO Pin 24 & ECHO on GPIO Pin 23.
    let mut ultrasonic = HcSr04::new(24, 23, None)?;

    loop {
        match ultrasonic.measure_distance(Unit::Meters)? {
            Some(dist) => println!("Distance: {:.2}m", dist),
            None => println!("Object out of range"),
        }

        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
    }
}
