use std::{
    fmt,
    time::{Duration, SystemTime},
};

use log::info;
use rand::{thread_rng, Rng};
use time::OffsetDateTime;
use tokio::time::sleep;

const MIN_TEMP: f32 = 20.0;
const MAX_TEMP: f32 = 25.0;
const SLEEP_IN_MS: u64 = 5000;

#[derive(Debug)]
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

#[allow(dead_code)]
pub struct DeviceSimulator {
    values: Vec<Temperature>,
}

impl Default for DeviceSimulator {
    fn default() -> Self {
        let item = Temperature {
            value: 22.0,
            date_time: SystemTime::now(),
        };
        Self { values: vec![item] }
    }
}

impl DeviceSimulator {
    pub async fn start(&self) {
        loop {
            match self.get_last_item() {
                Some(item) => {
                    let new_item = &self.get_new_item(item);
                    info!("{}", new_item);
                }
                None => {
                    info!("There are currently no valuess");
                }
            }
            sleep(Duration::from_millis(SLEEP_IN_MS)).await;
        }
    }

    fn get_last_item(&self) -> Option<&Temperature> {
        self.values.last()
    }

    fn get_new_item(&self, _last_item: &Temperature) -> Temperature {
        let mut rng = thread_rng();
        let value = rng.gen_range(MIN_TEMP..MAX_TEMP);

        Temperature {
            value,
            date_time: SystemTime::now(),
        }
    }
}
