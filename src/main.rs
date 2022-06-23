// use rust_azure_iot_simulator::metrics_controller::start_meter;
// use rust_azure_iot_simulator::metrics_server::MetricsReceiver;
// use rust_azure_iot_simulator::proto::*;

use rust_azure_iot_simulator::{proto::*, metrics_server::start, tmp_sender::start_tmp_test};
use tokio::join;

#[tokio::main]
async fn main() {
    // Enables log macros
    env_logger::init();

    let grpc = start();
    let client = start_tmp_test();

    join!(grpc, client);

    // tokio::spawn(async {
    //     start().await;
    // });

    // let config = get_deserialized_configuration().expect("Could not deserialized configuration");

    // let mut simulator = DeviceSimulator::new(config).await;
    // simulator.start().await;
    // start_meter().await;
}
