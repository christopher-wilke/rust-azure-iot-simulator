use log::debug;

use crate::proto::metrics::v1::ResourceMetrics;

pub struct D2C_Extractor {
    pub raw_data: Vec<ResourceMetrics>,
    pub metric: Metric
}

impl Default for D2C_Extractor {
    fn default() -> Self {
        Self { 
            raw_data: Default::default(), 
            metric: Default::default() 
        }
    }
}

pub struct Metric {
    name: String
}

impl Default for Metric {
    fn default() -> Self {
        Self { name: Default::default() }
    }
}

impl D2C_Extractor {
    pub fn extract_from_stream(&mut self) {
        println!("{:?}", self.raw_data);
        self.get_metric_name();
        // let metric = Metric::default();
    }

    pub fn get_metric_name(&self) {
        if let Some(resource_metric) = self.raw_data.get(0) {
            println!("Running in get_metric_name");
            println!("{:?}", resource_metric);
        } else {
            debug!("no resource metrics available");
        };
        // let name = match self.raw_data.get(0) {
        //     Some(resource_metric) => return resource_metric,
        //     None => None
        // };
    }

}