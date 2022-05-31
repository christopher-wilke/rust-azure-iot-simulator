use rust_azure_iot_simulator::simulator::DeviceSimulator;

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::init();

    let mut device_simulator = DeviceSimulator::default();
    device_simulator.start().await;
}
