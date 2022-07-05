use crossbeam_channel::{Receiver, bounded, tick, select};
use futures_util::Stream;
use log::debug;
use opentelemetry::metrics::Unit;
use opentelemetry::sdk::metrics::{selectors, PushController};
use opentelemetry::{metrics::{Meter, MetricsError, self}, sdk::{export::metrics::stdout::ExportBatch}, global};
use opentelemetry_otlp::{ExportConfig, Protocol, WithExportConfig};
use std::time::Duration;

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        sender.send(()).expect("Error while sending");
    })?;
    Ok(receiver)
}

pub struct MetricsController {
    meter: Meter
}

pub fn custom_formatter(batch: ExportBatch) -> Result<String, MetricsError> {
    Ok(format!("{:?}", batch))
}

fn delayed_interval(duration: Duration) -> impl Stream<Item = tokio::time::Instant> {
    opentelemetry::util::tokio_interval_stream(duration)
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

#[tokio::main]
pub async fn main() {

    // Enabling log macros
    env_logger::init();

    let ctrlc_c_events = ctrl_channel().expect("Could not create channel");
    let ticks = tick(Duration::from_secs(3));

    loop {
        select! {
            recv(ticks) -> _ => {
                gather_data();
            }
            recv(ctrlc_c_events) -> _event => {
                println!("received SIGINT event");
                break;
            }
        }
    }
}

pub fn gather_data() {
    let _ = init_meter().expect("Error while trying to create meter");
    let meter = global::meter("rust-azure-iot-simulator");

    let _ = meter
        .f64_value_observer(
            "temperature", 
            |r| r.observe(1337.6, &[])
        )
        .with_unit(Unit::new("Celsius"))
        .with_description("Current Temperature")
        .init();
}
