use std::{net::Ipv4Addr, error::Error};
use log::debug;
use tokio::{signal::unix::{signal, SignalKind}, select};
use tonic::{async_trait, Response, transport::Server};

use crate::proto::collector::metrics::v1::{metrics_service_server::{MetricsService, MetricsServiceServer}, ExportMetricsServiceRequest};


// OTEL Metrics Importer
#[derive(Debug)]
pub struct MetricsEndpoint {}

impl MetricsEndpoint {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl MetricsService for MetricsEndpoint {
    async fn export(&self, request:tonic::Request<ExportMetricsServiceRequest>,) -> Result<tonic::Response<crate::proto::collector::metrics::v1::ExportMetricsServiceResponse>,tonic::Status> {
        let request = request.get_ref();
        println!("Incoming request... {:?}", request);
        Ok(Response::new(crate::proto::collector::metrics::v1::ExportMetricsServiceResponse {}))
    }
}

async fn shutdown_signal() {
    let mut sigint =
        signal(SignalKind::interrupt()).expect("failed to setup SIGINT signal handler");
    let mut sighup = signal(SignalKind::hangup()).expect("failed to setup SIGHUP signal handler");
    let mut sigterm =
        signal(SignalKind::terminate()).expect("failed to setup SIGTERM signal handler");

    select! {
        _ = sigint.recv() => debug!("received SIGINT"),
        _ = sighup.recv() => debug!("received SIGHUP"),
        _ = sigterm.recv() => debug!("received SIGTERM"),
    };
}

pub async fn start() {
    let metrics_importer = MetricsEndpoint::new();
    let addr = (Ipv4Addr::UNSPECIFIED, 3880).into();

    debug!("listening for incoming grpc requests");

    let shutdown_handler = async {
        debug!("waiting for shutdown signal...");
        shutdown_signal().await;
        debug!("Shutting down...");
    };

    println!("Starting grpc server...");

    let svc = Server::builder()
        .add_service(MetricsServiceServer::new(metrics_importer))
        .serve(addr)
        .await
        .unwrap();

    // Server::builder()
    //     .concurrency_limit_per_connection(32)
    //     .add_service(MetricsServiceServer::new(metrics_importer))
    //     .serve_with_shutdown(addr, shutdown_handler)
    //     .await
    //     .expect("Failed to start the server");
}