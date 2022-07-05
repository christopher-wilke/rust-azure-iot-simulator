use log::{info};

use crate::proto::metrics::v1::{ResourceMetrics, ScopeMetrics};

pub struct D2cExtractor {
    pub raw_data: Vec<ResourceMetrics>,
    pub scope_metric: Option<ScopeMetrics>
}

impl Default for D2cExtractor {
    fn default() -> Self {
        Self { 
            raw_data: Default::default(),
            scope_metric: Default::default()
        }
    }
}

impl D2cExtractor {

    pub fn extract_scope_metric_from_stream(&mut self) {
        let sm = self.get_scope_metric();
        if sm.is_some() {
            println!("{:?}", sm.unwrap());
        } else {
            info!("Could not exract ScopeMetrics from raw data");
        }
    }

    pub fn get_scope_metric(&self) -> Option<ScopeMetrics> {
        match self.raw_data.get(0) {
            Some(rm) => {
                match rm.scope_metrics.get(0) {
                    Some(sm) => Some(sm.to_owned()),
                    None => None,
                } 
            },
            None => None,
        }
    }
}