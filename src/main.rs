use std::net::Ipv4Addr;
use async_trait::async_trait;
use log::{info, debug, error};
use tokio::{
    select,
    signal::unix::{signal, SignalKind},
    time::sleep,
};
use tonic::{transport::Server, Response};
use rust_azure_iot_simulator::{proto::collector::metrics::v1::{metrics_service_server::{
    MetricsService, MetricsServiceServer
}, ExportMetricsServiceResponse, ExportMetricsServiceRequest}, sender::gather_data, data_extractor::DataExtractor, instrumentation_scope::convert_to_d2c_message, configuration::IoTHubConfig, exporter::Exporter};


pub struct MetricsEndpoint;

#[async_trait]
impl MetricsService for MetricsEndpoint {
    async fn export(
        &self,
        request:tonic::Request<ExportMetricsServiceRequest>
    ) 
    -> Result<tonic::Response<ExportMetricsServiceResponse>,tonic::Status> {
        debug!("Async trait: incoming {request:?}");
        
        match DataExtractor::new(request.into_inner().resource_metrics) {
            Ok(extractor) => match extractor.start() {
                Ok(instrumentation_scope) => {
                    let serialized_msg = convert_to_d2c_message(&instrumentation_scope)
                        .expect("Error while trying to convert to JSON message");

                    let settings = IoTHubConfig::new().unwrap();

                    let mut exporter = Exporter::new(
                        &settings.iothub.hostname,
                        &settings.device.device_id,
                        &settings.device.shared_access_key,
                    )
                    .await;

                    info!("Sending -> {serialized_msg}");

                    exporter
                        .send_message(serialized_msg.as_bytes().to_vec())
                        .await
                        .unwrap();
                }
                Err(e) => error!("{e:?}"),
            },
            Err(e) => debug!("{e:?}"),
        }

        Ok(Response::new(ExportMetricsServiceResponse {}))
    }
}

#[tokio::main]
pub async fn main() {
    env_logger::init();
    run_tonic().await
}

async fn shutdown_signal() {
    let mut sigint =
        signal(SignalKind::interrupt()).expect("failed to setup SIGINT signal handler");
    let mut sighup = signal(SignalKind::hangup()).expect("failed to setup SIGHUP signal handler");
    let mut sigterm =
        signal(SignalKind::terminate()).expect("failed to setup SIGTERM signal handler");

    select! {
        _ = sigint.recv() => info!("received SIGINT"),
        _ = sighup.recv() => info!("received SIGHUP"),
        _ = sigterm.recv() => info!("received SIGTERM"),
    };
}

async fn run_tonic() {
    let metrics_importer = MetricsEndpoint {};
    let addr = (Ipv4Addr::UNSPECIFIED, 4317).into();

    let shutdown_handler = async {
        println!("waiting for shutdown signal...");
        shutdown_signal().await;
        println!("Shutting down...");
    };

    tokio::spawn(async move {
        run_sender().await;
    });

    Server::builder()
        .concurrency_limit_per_connection(32)
        .add_service(MetricsServiceServer::new(metrics_importer))
        .serve_with_shutdown(addr, shutdown_handler)
        .await
        .unwrap();
}

async fn run_sender() {
    loop {
        gather_data();
        sleep(std::time::Duration::from_secs(5)).await;
    }
}