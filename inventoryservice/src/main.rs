mod internal;

use std::sync::Arc;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use internal::app::Service;
use internal::config::Config;
use servicelib::runtime::{
    config::{CallSemantics, ConfigLoader},
    environment::{
        RuntimeEnvironment,
        metrics::{Metrics, NoopMetricsEngine},
    },
    telemetry::opentelemetry::{Config as OpenTelemetryConfig, OpenTelemetry},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (config_path, values_path) =
        config_paths("./config/config.yaml", "./config/overrides.yaml");
    let telemetry = if std::env::var_os("SERVICELIB_OTEL_ENABLED").is_some() {
        Some(OpenTelemetry::install(
            OpenTelemetryConfig::from_environment("Inventory Service"),
        )?)
    } else if std::env::var_os("SERVICELIB_NOOP_TRACING").is_none() {
        tracing_subscriber::fmt::init();
        None
    } else {
        None
    };
    let noop_metrics = std::env::var_os("SERVICELIB_NOOP_METRICS").is_some();
    let metrics = if noop_metrics {
        Metrics::noop()
    } else {
        telemetry
            .as_ref()
            .map_or_else(Metrics::default, |telemetry| telemetry.metrics().clone())
    };
    let loader = ConfigLoader::load(
        Some(config_path),
        Some(values_path),
        Config::default(),
        &metrics,
        "Inventory Service",
    )?;
    let config = loader.current();
    let environment = if noop_metrics {
        telemetry.map_or_else(
            || RuntimeEnvironment::with_metrics(CallSemantics::FunctionCall, metrics),
            |telemetry| {
                RuntimeEnvironment::with_telemetry(
                    CallSemantics::FunctionCall,
                    Arc::new(NoopMetricsEngine::new()),
                    telemetry.clone(),
                    telemetry,
                )
            },
        )
    } else {
        telemetry.map_or_else(
            || RuntimeEnvironment::with_metrics(CallSemantics::FunctionCall, metrics),
            |telemetry| {
                RuntimeEnvironment::with_telemetry(
                    CallSemantics::FunctionCall,
                    telemetry.clone(),
                    telemetry.clone(),
                    telemetry,
                )
            },
        )
    };
    environment.publish_runtime_config(loader.runtime_config());
    Service::new(&config, environment, loader)
        .await?
        .run()
        .await
}

fn config_paths(default_config: &str, default_values: &str) -> (String, String) {
    let mut config = default_config.to_owned();
    let mut values = default_values.to_owned();
    let mut args = std::env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--config" => config = args.next().unwrap_or(config),
            "--values" => values = args.next().unwrap_or(values),
            _ => {}
        }
    }
    (config, values)
}
