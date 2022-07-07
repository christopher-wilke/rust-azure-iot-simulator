#[derive(Debug)]
pub struct InstrumentationScope {
    pub name: String,
    pub metric: InstrumentationMetric
}

#[derive(Debug)]
pub struct InstrumentationMetric {
    pub name: String,
    pub description: String,
    pub unit: String,
    pub data_point: InstrumentationDataPoint
}

#[derive(Debug)]
pub struct InstrumentationDataPoint {
    pub start_time_unix_nano: u64,
    pub time_unix_nano: u64,
    pub value: f64
}