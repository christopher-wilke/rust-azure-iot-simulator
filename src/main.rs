use rust_azure_iot_simulator::simulator::DeviceSimulator;

#[tokio::main]
async fn main() {
    let device_simulator = DeviceSimulator::new();
    device_simulator.start();
}
