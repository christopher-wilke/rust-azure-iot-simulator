use std::net::Ipv4Addr;

use async_trait::async_trait;
use log::{error, debug};
use receiver::{proto::{collector::metrics::v1::{*, metrics_service_server::{MetricsService, MetricsServiceServer}}}, data_extractor::DataExtractor, instrumentation_scope::convert_to_d2c_message, configuration::{IoTHubConfig, ConfigurationFile, ConfigurationFileError}, exporter::Exporter};
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
                        let serialized_msg = convert_to_d2c_message(&instrumentation_scope)
                            .expect("Error while trying to convert to JSON message");

                        let settings = IoTHubConfig::new().unwrap();

                        let mut exporter = Exporter::new(
                            &settings.iothub.hostname,
                            &settings.device.device_id,
                            &settings.device.shared_access_key
                        ).await;

                        println!("Sending -> {serialized_msg}");

                        exporter.send_message(
                            serialized_msg
                            .as_bytes()
                            .to_vec()
                        )
                        .await
                        .unwrap();
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
pub async fn main() -> Result<(), ConfigurationFileError> {
    env_logger::init();

    run_server().await;
    
    Ok(())
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