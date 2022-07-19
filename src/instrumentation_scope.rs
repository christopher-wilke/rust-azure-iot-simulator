use serde::{Deserialize, Serialize};
use serde_json::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct InstrumentationScope {
    pub name: String,
    pub metric: InstrumentationMetric,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstrumentationMetric {
    pub name: String,
    pub description: String,
    pub unit: String,
    pub data_point: InstrumentationDataPoint,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstrumentationDataPoint {
    pub start_time_unix_nano: u64,
    pub time_unix_nano: u64,
    pub value: f64,
    pub current_time_unix: u128,
}

pub fn convert_to_d2c_message<T>(instrumentation_scope: T) -> Result<String, Error>
where
    T: Serialize,
{
    serde_json::to_string(&instrumentation_scope)
}
