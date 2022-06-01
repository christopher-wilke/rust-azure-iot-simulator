use std::time::{Duration, SystemTime};

use azure_iot_sdk::{DeviceKeyTokenSource, IoTHubClient, Message};
use log::debug;
use time::OffsetDateTime;
use tokio::time::sleep;

pub async fn send_d2c_message(hostname: &str, device_id: &str, shared_access_key: &str, msg: Vec<u8>) {

    let token_source = DeviceKeyTokenSource::new(hostname, device_id, shared_access_key)
        .expect("Could not create Token Source");

    let mut client = IoTHubClient::new(hostname, device_id.to_string(), token_source)
        .await
        .expect("Could not create an instance of IoTHubClient");

    debug!("IoT Hub successfully initialized");

    match client.send_message(Message::new(msg)).await {
        Ok(_) => debug!("Message sent"),
        Err(_) => debug!("Could not send message"),
    }

    sleep(Duration::from_secs(5)).await;
}

fn get_local_time() -> String {
    let local_time: OffsetDateTime = SystemTime::now().into();
    format!(
        "{}:{}:{}",
        local_time.hour(),
        local_time.minute(),
        local_time.second()
    )
}
