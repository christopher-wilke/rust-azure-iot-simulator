use rust_azure_iot_simulator::simulator::DeviceSimulator;

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::init();

    let device_simulator = DeviceSimulator::new();
    device_simulator.start().await;
}
