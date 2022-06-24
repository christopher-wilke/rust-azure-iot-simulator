use std::net::Ipv4Addr;

use async_trait::async_trait;
use receiver::proto::collector::metrics::v1::{*, metrics_service_server::{MetricsService, MetricsServiceServer}};
use tonic::{transport::Server, Response};

pub struct MetricsEndpoint {}

impl MetricsEndpoint {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl MetricsService for MetricsEndpoint {
    async fn export(
        &self,
        request: tonic::Request<ExportMetricsServiceRequest>,
    ) -> Result<Response<ExportMetricsServiceResponse>, tonic::Status> {
       let data = request.into_inner();
       println!("{:?}", data);
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