use std::time::Duration;

use futures_util::Stream;
use log::{debug, info};
use opentelemetry::{metrics::{Meter, MetricsError, self, Unit}, sdk::{export::metrics::stdout::ExportBatch, metrics::{PushController, selectors}}, global};
use opentelemetry_otlp::{ExportConfig, Protocol, WithExportConfig};

use crate::simulator::get_new_item;

pub struct MetricsController {
    meter: Meter
}

pub fn custom_formatter(batch: ExportBatch) -> Result<String, MetricsError> {
    Ok(format!("{:?}", batch))
}

impl MetricsController {
    pub fn default() -> Self {
        opentelemetry::sdk::export::metrics::stdout(tokio::spawn, delayed_interval)
            .with_formatter(custom_formatter)
            .init();
        
        let meter = global::meter("rust-azure-iot-simulator");
        Self { meter }
    }
}

fn delayed_interval(duration: Duration) -> impl Stream<Item = tokio::time::Instant> {
    opentelemetry::util::tokio_interval_stream(duration)
}

fn init_meter() -> metrics::Result<PushController> {

    let export_config = ExportConfig {
        endpoint: "http://localhost:4317".into(),
        protocol: Protocol::Grpc,
        ..ExportConfig::default()
    };

    opentelemetry_otlp::new_pipeline()
        .metrics(tokio::spawn, delayed_interval)
        .with_exporter(
            opentelemetry_otlp::new_exporter()
            .tonic()
            .with_export_config(export_config)
        )
        .with_aggregator_selector(selectors::simple::Selector::Exact)
        .build()
}

pub fn gather_data() {
    let _ = init_meter().expect("Error while trying to create meter");
    let meter = global::meter("rust-azure-iot-simulator");
    let random_value = get_new_item().value;

    info!("Generated random value: {random_value}");

    let _ = meter
        .f64_value_observer(
            "temperature", 
            move |r| r.observe(random_value, &[])
        )
        .with_unit(Unit::new("Celsius"))
        .with_description("Current Temperature")
        .init();
}