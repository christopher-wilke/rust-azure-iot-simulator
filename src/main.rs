use rust_azure_iot_simulator::{sender::send_d2c_message};

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::init();

    // let mut device_simulator = DeviceSimulator::default();
    // device_simulator.start().await;

    send_d2c_message().await;
}
