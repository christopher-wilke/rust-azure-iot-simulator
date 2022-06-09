use rust_azure_iot_simulator::{
    metrics_controller::init_meter,
    // simulator::DeviceSimulator,
};

#[tokio::main]
async fn main() {
    // Enables log macros
    env_logger::init();

    // let config = get_deserialized_configuration().expect("Could not deserialized configuration");

    // let mut simulator = DeviceSimulator::new(config).await;
    // simulator.start().await;
    init_meter();
}
