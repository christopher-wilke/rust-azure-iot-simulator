use log::debug;
use rust_azure_iot_simulator::{configuration::get_deserialized_configuration, simulator::DeviceSimulator, sender::send_d2c_message};

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::init();

    // Synchronously read `base.yaml` from `configuration` folder
    let config = get_deserialized_configuration().expect("Could not load config file");
    let mut simulator = DeviceSimulator::new(config);

    simulator.start().await;

    // let mut device_simulator = DeviceSimulator::default()
    //     .start()
    //     .await;

    // send_d2c_message(
    //     &config.iothub.hostname,
    //     &config.device.device_id,
    //     &config.device.shared_access_key
    // )
    // .await;
}
