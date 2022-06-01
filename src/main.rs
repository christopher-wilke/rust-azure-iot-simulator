use log::debug;
use rust_azure_iot_simulator::{configuration::{get_deserialized_configuration}};

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::init();

    // Read `base.yaml` from `configuration` folder
    let config = get_deserialized_configuration().expect("Could not load config file");

    debug!("{}", config.iothub.hostname);

    // let mut device_simulator = DeviceSimulator::default();
    // device_simulator.start().await;

    // send_d2c_message().await;

  
}
