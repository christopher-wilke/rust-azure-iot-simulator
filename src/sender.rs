use std::time::SystemTime;

use azure_iot_sdk::{DeviceKeyTokenSource, IoTHubClient, Message};
use time::OffsetDateTime;

pub struct Sender {
    client: IoTHubClient,
}

impl Sender {
    pub async fn new(hostname: &str, device_id: &str, shared_access_key: &str) -> Self {
        let token_source = DeviceKeyTokenSource::new(hostname, device_id, shared_access_key)
            .expect("Could not create Token Source");

        let client = IoTHubClient::new(hostname, device_id.to_string(), token_source)
            .await
            .expect("Could not create an instance of IoTHubClient");

        Self { client }
    }

    pub async fn send_message(&mut self, msg: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        self.client.send_message(Message::new(msg)).await
    }
}

fn _get_local_time() -> String {
    let local_time: OffsetDateTime = SystemTime::now().into();
    format!(
        "{}:{}:{}",
        local_time.hour(),
        local_time.minute(),
        local_time.second()
    )
}
