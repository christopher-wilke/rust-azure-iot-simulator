use std::net::Ipv4Addr;

use async_trait::async_trait;
use receiver::proto::{collector::metrics::v1::{*, metrics_service_server::{MetricsService, MetricsServiceServer}}, metrics::v1::{Gauge, metric::Data}};
use tonic::{transport::Server, Response};

pub struct MetricsEndpoint {}

impl MetricsEndpoint {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_d2c_message() {

    }
}

#[async_trait]
impl MetricsService for MetricsEndpoint {
    async fn export(
        &self,
        request: tonic::Request<ExportMetricsServiceRequest>,
    ) -> Result<Response<ExportMetricsServiceResponse>, tonic::Status> {
        
        let data = request.into_inner().resource_metrics;

        match data.get(0) {
            Some(resource_metrics) => {
                let scope_metrics = &resource_metrics.scope_metrics.get(0).unwrap();
                unsafe { 
                    let metric = scope_metrics.metrics.get_unchecked(0).to_owned();

                    match metric.data {
                        Some(Data::Gauge(gauge)) => {
                            println!{"{:?}", gauge.data_points.get(0).unwrap().value};
                        },
                        Some(_) => {},
                        None => todo!(),
                    };
                    // let foo = match &val.data {
                    //     Some(Data::Gauge(gauge)) => {
                           
                    //     },
                    //     None => {
                    //         panic!("paniced");
                    //     }
                    // };

                    // println!("{:?}", val.data);
                    // match &val.data {
                    //     Some(value) => {
                    //        let new_val = value.to_owned();
                    //     }
                    //     None => {}
                    // }
                }   
            },
            None => {}
        }

       Ok(Response::new(ExportMetricsServiceResponse {}))
    }
}

#[tokio::main]
pub async fn main() {
    run_server().await;
}

async fn run_server() {

    let metrics_importer = MetricsEndpoint::new();
    let addr = (Ipv4Addr::UNSPECIFIED, 4317).into();

    Server::builder()
        .concurrency_limit_per_connection(32)
        .add_service(MetricsServiceServer::new(metrics_importer))
        .serve(addr)
        .await
        .expect("Error while trying to run grpc server");
}