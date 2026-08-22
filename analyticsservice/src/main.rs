mod internal;

use std::sync::Arc;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use internal::{app::Service, config::Config};
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
    let telemetry = if std::env::var_os("SERVICELIB_OTEL_ENABLED").is_some() {
        Some(OpenTelemetry::install(
            OpenTelemetryConfig::from_environment("Analytics Service"),
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
        Some("./config/config.yaml"),
        Some("./config/overrides.yaml"),
        Config::default(),
        &metrics,
        "Analytics Service",
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
    Service::new(&config, environment, loader)?.run().await
}
