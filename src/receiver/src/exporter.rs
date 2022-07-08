use azure_iot_sdk::{IoTHubClient, DeviceKeyTokenSource, Message};

pub struct Exporter {
    client: IoTHubClient
}

impl Exporter {
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