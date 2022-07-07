use std::net::Ipv4Addr;

use async_trait::async_trait;
use log::error;
use receiver::{proto::{collector::metrics::v1::{*, metrics_service_server::{MetricsService, MetricsServiceServer}}}, data_extractor::DataExtractor};
use tonic::{transport::Server, Response};

pub struct MetricsEndpoint;

#[async_trait]
impl MetricsService for MetricsEndpoint {
    async fn export(
        &self,
        request: tonic::Request<ExportMetricsServiceRequest>,
    ) -> Result<Response<ExportMetricsServiceResponse>, tonic::Status> {

        match DataExtractor::new(
            request
            .into_inner()
            .resource_metrics
        ) {
            Ok(extractor) => {
                let state = extractor.start();
            },
            Err(e) => error!("{e:?}"),
        }

        // let mut extractor = D2cExtractor {
        //     raw_data: request
        //         .into_inner()
        //         .resource_metrics,
        //     ..Default::default()
        // };

        // extractor.extract_scope_metric_from_stream();

        // message.extract_from_stream();

        // let data = request.into_inner().resource_metrics;

        // match data.get(0) {
        //     Some(resource_metrics) => {
        //         let scope_metrics = &resource_metrics.scope_metrics.get(0).unwrap();
        //         // unsafe { 
        //         //     let metric = scope_metrics.metrics.get_unchecked(0).to_owned();

        //         //     match metric.data {
        //         //         Some(Data::Gauge(gauge)) => {
        //         //             println!{"{:?}", gauge.data_points.get(0).unwrap().value};
        //         //         },
        //         //         Some(_) => {},
        //         //         None => todo!(),
        //         //     };
        //         // }   
        //     },
        //     None => {}
        // }

       Ok(Response::new(ExportMetricsServiceResponse {}))
    }
}

#[tokio::main]
pub async fn main() {
    env_logger::init();

    run_server().await;
}

async fn run_server() {

    let metrics_importer = MetricsEndpoint {};
    let addr = (Ipv4Addr::UNSPECIFIED, 4317).into();

    Server::builder()
        .concurrency_limit_per_connection(32)
        .add_service(MetricsServiceServer::new(metrics_importer))
        .serve(addr)
        .await
        .expect("Error while trying to run grpc server");
}