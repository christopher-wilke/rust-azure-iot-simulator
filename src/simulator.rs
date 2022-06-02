use std::{
    fmt,
    time::{Duration, SystemTime},
};

use log::{error, info};
use rand::{thread_rng, Rng};
use time::OffsetDateTime;
use tokio::time::sleep;

use crate::{configuration::Settings, sender::send_d2c_message};

const MIN_TEMP: f32 = 20.0;
const MAX_TEMP: f32 = 25.0;
const SLEEP_IN_MS: u64 = 5000;

#[derive(Debug, Clone)]
pub struct Temperature {
    value: f32,
    date_time: SystemTime,
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let locale_time: OffsetDateTime = self.date_time.into();
        let now = format!(
            "{}:{}:{}",
            locale_time.hour(),
            locale_time.minute(),
            locale_time.second()
        );
        write!(f, "{:?}: {} Celsius", now, &self.value)
    }
}

pub struct DeviceSimulator {
    // we currently hold the values Vec for future improvements
    values: Vec<Temperature>,
    settings: Settings,
}

impl DeviceSimulator {
    pub fn new(settings: Settings) -> Self {
        let temperature = Temperature {
            value: 22.,
            date_time: SystemTime::now(),
        };

        Self {
            values: vec![temperature],
            settings,
        }
    }

    pub async fn start(&mut self) {
        loop {
            match self.get_last_item() {
                Some(_item) => {
                    let data = &self.get_new_item();
                    let values = self.insert_data(data.to_owned());
                    info!(
                        "Current Size: {}, Last Item: {:?}",
                        values.len(),
                        values.last().expect("Array is empty")
                    );

                    // Converting
                    let last_item = values
                        .last()
                        .expect("Array is empty")
                        .to_string()
                        .as_bytes()
                        .to_vec();

                    send_d2c_message(
                        &self.settings.iothub.hostname,
                        &self.settings.device.device_id,
                        &self.settings.device.shared_access_key,
                        last_item,
                    )
                    .await;
                }
                None => error!("Values Array seems to be empty"),
            }
            sleep(Duration::from_millis(SLEEP_IN_MS)).await;
        }
    }

    fn insert_data(&mut self, data: Temperature) -> &Vec<Temperature> {
        self.values.push(data);
        &self.values
    }

    fn get_last_item(&self) -> Option<&Temperature> {
        self.values.last()
    }

    fn get_new_item(&self) -> Temperature {
        let mut rng = thread_rng();
        let value = rng.gen_range(MIN_TEMP..MAX_TEMP);

        Temperature {
            value,
            date_time: SystemTime::now(),
        }
    }
}
