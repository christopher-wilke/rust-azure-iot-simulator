use opentelemetry::{global, KeyValue};

pub fn init_meter() {
    // Get a meter from the provider
    let meter = global::meter("rust-azure-iot-simulator");

    // Create an Instrument
    let counter = meter.u64_counter("chriwils-counter").init();

    // Record a measurement
    counter.add(1, &[KeyValue::new("http.client_ip", "83.164.160.102")]);
    counter.add(1, &[KeyValue::new("http.client_ip", "83.164.160.102")]);
    counter.add(1, &[KeyValue::new("http.client_ip", "83.164.160.102")]);
    counter.add(1, &[KeyValue::new("http.client_ip", "83.164.160.102")]);

    println!("{:?}", counter);
}