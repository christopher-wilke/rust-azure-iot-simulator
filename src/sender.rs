use std::time::{SystemTime, Duration};

use azure_iot_sdk::{DeviceKeyTokenSource, IoTHubClient, Message};
use log::debug;
use time::OffsetDateTime;
use tokio::time::sleep;

pub async fn send_d2c_message() {
    let hostname = "hostname";
    let device_id = "device_id";
    let shared_access_key = "sas_key";

    let token_source = DeviceKeyTokenSource::new(
        hostname,
        device_id,
        shared_access_key
    )
    .expect("Could not create Token Source");

    let mut client = IoTHubClient::new(
        &hostname,
        device_id.to_string(),
        token_source
    )
    .await
    .expect("Could not create an instance of IoTHubClient");

    debug!("IoT Hub successfully initialized");

    loop {
        
        let body = format!("{} -  Hello from device", get_local_time());
        let message = Message::new(body.as_bytes().to_vec());
        debug!("{:?}", message);

        match client.send_message(message).await {
            Ok(_) => debug!("Message sent"),
            Err(_) => debug!("Could not send message")
        }

        sleep(Duration::from_secs(5)).await;
    }
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