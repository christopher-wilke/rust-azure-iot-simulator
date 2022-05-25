pub struct Temperature {
    value: f32,
    unit: String
}

pub struct DeviceSimulator {
    temperature: Temperature
}

impl DeviceSimulator {
    fn new() -> Self {
        Self {}
    }
}