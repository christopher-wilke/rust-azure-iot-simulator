use log::{info};

use crate::proto::metrics::v1::{ResourceMetrics, ScopeMetrics, Gauge, metric::Data, number_data_point::Value};

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

            // let name = self.get_scope_metric_name(sm.as_ref().unwrap());
            // println!("{:?}", sm);

            // let metrics = sm.unwrap().metrics;
            self.get_metric_data_point(sm.as_ref().unwrap());

            // let name = sm
            //     .as_ref()
            //     .unwrap();
            
            // let metrics = &sm
            //     .unwrap()
            //     .metrics;
            
        } else {
            info!("Could not extract ScopeMetrics from raw data");
        }
    }

    fn get_metric_data_point(&self, scope_metric: &ScopeMetrics) -> Option<String> {
        match *(&scope_metric.metrics.get(0)) {
            Some(metric) => {
                match &metric.data {
                    Some(Data::Gauge(gauge)) => {
                       match *(&gauge.data_points.get(0)) {
                            Some(dp) => {
                                match dp.value.as_ref().unwrap() {
                                    Value::AsDouble(v) => {
                                        println!("{}", v);
                                    }
                                    _ => return None
                                }
                                return None
                            },
                            None => return None
                        };                       
                    },
                    _ => return None
                }
            },
            None => return None,
        }
    }

    fn get_scope_metric_name(&self, scope_metric: &ScopeMetrics) -> Option<String> {
        if let Some(scope) = &scope_metric.scope {
            Some(scope.name.to_owned())
        } else {
            None
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