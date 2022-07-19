use std::{error::Error, fmt::Display, time::{SystemTime, UNIX_EPOCH}};

use error_stack::{Report, Result, ResultExt};

use crate::{
    instrumentation_scope::{
        InstrumentationDataPoint, InstrumentationMetric, InstrumentationScope,
    },
    proto::metrics::v1::{metric::Data, number_data_point::Value, ResourceMetrics},
};

#[derive(Debug)]
pub struct ResourceMetricError;

impl Display for ResourceMetricError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Error while trying to extract Resource Metric")
    }
}

impl Error for ResourceMetricError {}

#[derive(Debug)]
pub struct DataExtractor {
    resource_metrics: ResourceMetrics,
}

impl DataExtractor {
    pub fn new(raw_data: Vec<ResourceMetrics>) -> Result<DataExtractor, ResourceMetricError> {
        match raw_data.get(0) {
            Some(rm) => Ok(Self {
                resource_metrics: rm.clone(),
            }),
            None => Err(Report::new(ResourceMetricError)).attach_printable("raw_data is empty"),
        }
    }

    pub fn start(&self) -> Result<InstrumentationScope, ResourceMetricError> {
        Ok(InstrumentationScope {
            name: self.get_name()?,
            metric: self.get_metric()?,
        })
    }

    fn get_name(&self) -> Result<String, ResourceMetricError> {
        match self.resource_metrics.scope_metrics.get(0) {
            Some(scope_metric) => match &scope_metric.scope {
                Some(scope) => Ok(scope.name.to_owned()),
                None => Err(Report::new(ResourceMetricError))
                    .attach_printable("No scope in scope_metric available"),
            },
            None => Err(Report::new(ResourceMetricError))
                .attach_printable("Cannot access scope metrics"),
        }
    }

    fn get_metric(&self) -> Result<InstrumentationMetric, ResourceMetricError> {
        match self.resource_metrics.scope_metrics.get(0) {
            Some(scope_metric) => match scope_metric.metrics.get(0) {
                Some(metric) => Ok(InstrumentationMetric {
                    name: metric.name.to_owned(),
                    description: metric.description.to_owned(),
                    unit: metric.unit.to_owned(),
                    data_point: self.get_data_point()?,
                }),
                None => {
                    Err(Report::new(ResourceMetricError)).attach_printable("No metric available")
                }
            },
            None => Err(Report::new(ResourceMetricError))
                .attach_printable("No scope in scope_metric available"),
        }
    }

    pub fn current_time(&self) -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("could not retrieve current system date time")
            .as_millis()
    }

    fn get_data_point(&self) -> Result<InstrumentationDataPoint, ResourceMetricError> {
        match self.resource_metrics.scope_metrics.get(0) {
            Some(scope_metric) => match scope_metric.metrics.get(0) {
                Some(metric) => match &metric.data {
                    Some(Data::Gauge(gauge)) => match gauge.data_points.get(0) {
                        Some(dp) => match dp.value {
                            Some(Value::AsDouble(value)) => Ok(InstrumentationDataPoint {
                                start_time_unix_nano: dp.start_time_unix_nano,
                                time_unix_nano: dp.time_unix_nano,
                                value,
                                current_time_unix: self.current_time()
                            }),
                            _ => Err(Report::new(ResourceMetricError))
                                .attach_printable("Value (f64) not available"),
                        },
                        None => Err(Report::new(ResourceMetricError))
                            .attach_printable("Could not access gauge data point"),
                    },
                    _ => Err(Report::new(ResourceMetricError))
                        .attach_printable("Could not access Gauge data"),
                },
                None => {
                    Err(Report::new(ResourceMetricError)).attach_printable("No metric available")
                }
            },
            None => Err(Report::new(ResourceMetricError))
                .attach_printable("No scope in scope_metric available"),
        }
    }
}

