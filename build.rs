use anyhow::Context;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

const SOURCE_DIR: &str = "lib/opentelemetry-proto";
const PREPROCESS_DIR: &str = "preprocess";
const PROTO_OUT_DIR: &str = "proto";

const OTEL_PROTOS: &[&str] = &[
    "opentelemetry/proto/common/v1/common.proto",
    "opentelemetry/proto/resource/v1/resource.proto",
    "opentelemetry/proto/trace/v1/trace.proto",
    "opentelemetry/proto/trace/v1/trace_config.proto",
    "opentelemetry/proto/collector/trace/v1/trace_service.proto",
    "opentelemetry/proto/metrics/v1/metrics.proto",
    "opentelemetry/proto/collector/metrics/v1/metrics_service.proto",
    "opentelemetry/proto/logs/v1/logs.proto",
    "opentelemetry/proto/collector/logs/v1/logs_service.proto",
];

const OTEL_INCLUDES: &[&str] = &["."];

const SERIALIZABLE_TYPES: &[&str] = &[
    "opentelemetry.proto.common.v1.KeyValue",
    "opentelemetry.proto.common.v1.ArrayValue",
    "opentelemetry.proto.common.v1.KeyValueList",
    "opentelemetry.proto.common.v1.StringKeyValue",
    "opentelemetry.proto.common.v1.AnyValue",
    "opentelemetry.proto.common.v1.AnyValue.value",
    "opentelemetry.proto.common.v1.InstrumentationLibrary",
    "opentelemetry.proto.common.v1.InstrumentationScope",
    "opentelemetry.proto.resource.v1.Resource",
    "opentelemetry.proto.trace.v1.ResourceSpans",
    "opentelemetry.proto.trace.v1.InstrumentationLibrarySpans",
    "opentelemetry.proto.trace.v1.ScopeSpans",
    "opentelemetry.proto.trace.v1.Span",
    "opentelemetry.proto.trace.v1.Span.Event",
    "opentelemetry.proto.trace.v1.Span.Link",
    "opentelemetry.proto.trace.v1.Status",
    "opentelemetry.proto.collector.trace.v1.ExportTraceServiceRequest",
    "opentelemetry.proto.metrics.v1.ResourceMetrics",
    "opentelemetry.proto.metrics.v1.InstrumentationLibraryMetrics",
    "opentelemetry.proto.metrics.v1.ScopeMetrics",
    "opentelemetry.proto.metrics.v1.Metric",
    "opentelemetry.proto.metrics.v1.Metric.data",
    "opentelemetry.proto.metrics.v1.IntGauge",
    "opentelemetry.proto.metrics.v1.Gauge",
    "opentelemetry.proto.metrics.v1.IntSum",
    "opentelemetry.proto.metrics.v1.Sum",
    "opentelemetry.proto.metrics.v1.IntHistogram",
    "opentelemetry.proto.metrics.v1.Histogram",
    "opentelemetry.proto.metrics.v1.ExponentialHistogram",
    "opentelemetry.proto.metrics.v1.Summary",
    "opentelemetry.proto.metrics.v1.NumberDataPoint",
    "opentelemetry.proto.metrics.v1.NumberDataPoint.value",
    "opentelemetry.proto.metrics.v1.HistogramDataPoint",
    "opentelemetry.proto.metrics.v1.ExponentialHistogramDataPoint",
    "opentelemetry.proto.metrics.v1.ExponentialHistogramDataPoint.Buckets",
    "opentelemetry.proto.metrics.v1.SummaryDataPoint",
    "opentelemetry.proto.metrics.v1.SummaryDataPoint.ValueAtQuantile",
    "opentelemetry.proto.metrics.v1.Exemplar",
    "opentelemetry.proto.metrics.v1.Exemplar.value",
    "opentelemetry.proto.metrics.v1.IntDataPoint",
    "opentelemetry.proto.metrics.v1.IntExemplar",
    "opentelemetry.proto.metrics.v1.IntHistogramDataPoint",
    "opentelemetry.proto.collector.metrics.v1.ExportMetricsServiceRequest",
    "opentelemetry.proto.logs.v1.ResourceLogs",
    "opentelemetry.proto.logs.v1.InstrumentationLibraryLogs",
    "opentelemetry.proto.logs.v1.ScopeLogs",
    "opentelemetry.proto.logs.v1.LogRecord",
    "opentelemetry.proto.collector.logs.v1.ExportLogsServiceRequest",
];

const DEPRECATED_FIELDS: &[&str] = &[
    "opentelemetry.proto.metrics.v1.Metric.data.int_gauge",
    "opentelemetry.proto.metrics.v1.Metric.data.int_sum",
    "opentelemetry.proto.metrics.v1.Metric.data.int_histogram",
];

fn main() -> anyhow::Result<()> {
    let out_dir = std::env::var("OUT_DIR").context("Missing output directory")?;

    // Preprocess all protobuf files
    let preprocess_dir: PathBuf = [&out_dir, PREPROCESS_DIR].into_iter().collect();
    preprocess(Path::new(SOURCE_DIR), &preprocess_dir, OTEL_PROTOS)
        .context("error preprocessing protobuf files")?;

    // Create the output directory
    let out_dir: PathBuf = [&out_dir, PROTO_OUT_DIR].into_iter().collect();
    std::fs::create_dir_all(&out_dir).context("Error creating output directory")?;

    // Configure the builder
    let builder = tonic_build::configure()
        .build_client(false)
        .out_dir(out_dir);

    // Apply attributes to types
    let builder = SERIALIZABLE_TYPES.iter().fold(builder, |builder, &path| {
        builder.type_attribute(path, "#[derive(serde::Serialize)]")
    });
    let builder = DEPRECATED_FIELDS.iter().fold(builder, |builder, &path| {
        builder.field_attribute(path, "#[deprecated]")
    });

    // Compile the protobuf files
    let protos: Vec<_> = OTEL_PROTOS
        .iter()
        .map(|path| preprocess_dir.join(path))
        .collect();
    let includes: Vec<_> = OTEL_INCLUDES
        .iter()
        .map(|path| preprocess_dir.join(path))
        .collect();
    builder
        .compile(&protos, &includes)
        .context("Error compiling protobuf files")?;

    Ok(())
}

/// Preprocesses the protobuf files. This modifies the comments in each protobuf
/// file to be surrounded by text blocks. Many of the files use indented
/// comments which translate to code blocks in Markdown, causing `cargo test` to
/// assume those are documentation tests.
fn preprocess(
    input_dir: &Path,
    output_dir: &Path,
    protos: impl IntoIterator<Item = impl AsRef<Path>>,
) -> anyhow::Result<()> {
    // Create output directory
    std::fs::create_dir_all(output_dir).context("error creating preprocessor output directory")?;

    // Convert all the comments into this format:
    // ```text
    // <comment text>
    // ```
    for path in protos {
        // Open source file
        let input_path = input_dir.join(path.as_ref());
        let input_file = OpenOptions::new()
            .read(true)
            .open(&input_path)
            .with_context(|| format!("error opening input file '{}'", input_path.display()))?;
        let input_file = BufReader::new(input_file);

        // Open output file
        let output_path = output_dir.join(path.as_ref());
        let output_path_dir = output_path.parent().context("invalid output path")?;
        std::fs::create_dir_all(output_path_dir).with_context(|| {
            format!(
                "error creating output directory '{}'",
                output_path_dir.display()
            )
        })?;
        let mut output_file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&output_path)
            .with_context(|| format!("error opening output file '{}'", output_path.display()))?;

        // Process lines
        let mut was_comment = false;
        for line in input_file.lines() {
            let line =
                line.with_context(|| format!("error reading line in '{}'", input_path.display()))?;
            let is_comment = line.trim_start().starts_with("//");

            match (is_comment, was_comment) {
                // Open text block
                (true, false) => writeln!(output_file, "// ```text")?,
                // Close text block
                (false, true) => writeln!(output_file, "// ```")?,
                _ => {}
            };

            // Write line
            writeln!(output_file, "{}", line)?;

            // Keep track of whether the last read line was a comment
            was_comment = is_comment;
        }

        // Close text block if last line of file was a comment
        if was_comment {
            write!(output_file, "// ```")?;
        }
    }

    Ok(())
}
