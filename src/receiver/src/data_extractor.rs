use std::{fmt::Display, error::Error};

use error_stack::{IntoReport, Report, Result, ResultExt};

use crate::proto::metrics::v1::ResourceMetrics;

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
    resource_metrics: ResourceMetrics
}

impl DataExtractor {
    pub fn new(raw_data: Vec<ResourceMetrics>) -> Result<DataExtractor, ResourceMetricError> {
        match raw_data.get(0) {
            Some(rm) => {
                Ok(Self {
                    resource_metrics: rm.clone()
                })
            },
            None => Err(Report::new(ResourceMetricError))
                        .attach_printable(format!("raw_data is empty"))
        }
    }

    pub fn start(&self) -> Result<String, ResourceMetricError> {
        let name = self.get_name()?;
        println!("name = {}", name);
        
        Ok("extraction successful".into())
    }

    fn get_name(&self) -> Result<String, ResourceMetricError> {
        match self.resource_metrics.scope_metrics.get(0) {
            Some(scope_metric) => {
                match &scope_metric.scope {
                    Some(scope) => Ok(scope.name.to_owned()),
                    None => Err(Report::new(ResourceMetricError))
                        .attach_printable(("No scope in scope_metric available"))
                }
            },
            None => Err(Report::new(ResourceMetricError))
                .attach_printable(("Cannot access scope metrics"))
        }
    }

}