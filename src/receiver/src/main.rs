use std::net::Ipv4Addr;

use async_trait::async_trait;
use log::{error, info, debug};
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
                match extractor.start() {
                    Ok(instrumentation_scope) => {
                        let serialized_scope = serde_json::to_string(&instrumentation_scope).unwrap();
                        println!("{serialized_scope}");
                    },
                    Err(e) => error!("{e:?}"),
                }
            },
            Err(e) => debug!("{e:?}"),
        }

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