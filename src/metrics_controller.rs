use futures_util::Stream;
use opentelemetry::{
    global,
    metrics::{Meter, MetricsError, ObserverResult},
    sdk::export::metrics::stdout::ExportBatch,
};
use std::time::Duration;

pub struct MetricsController {
    meter: Meter,
}

fn delayed_interval(duration: Duration) -> impl Stream<Item = tokio::time::Instant> {
    opentelemetry::util::tokio_interval_stream(duration)
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

pub async fn start_meter() {
    loop {
        let controller = MetricsController::default();
        let one_metric_callback = |res: ObserverResult<f64>| res.observe(1.0, &[]);

        let _ = controller
            .meter
            .f64_value_observer("temperature", one_metric_callback)
            .init();

        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}
