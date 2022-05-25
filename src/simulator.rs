use std::{time::SystemTime};

#[allow(dead_code)]
enum TemperatureUnit {
    Celsius,
    Fahrenheit
}

#[allow(dead_code)]
pub struct Temperature {
    value: f32,
    unit: TemperatureUnit,
    date_time: SystemTime
}

#[allow(dead_code)]
pub struct DeviceSimulator {
    values: Vec<Temperature>
}

impl DeviceSimulator {
    pub fn new() -> Self {
        Self { values: vec![] }
    }

    pub fn start(&self) {
        println!("hello from start method :)");
    }
}