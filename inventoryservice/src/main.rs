use inventory_service::internal;

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
    telemetry::opentelemetry::{
        Config as OpenTelemetryConfig, OpenTelemetry, environment_flag_enabled, install_stdout,
    },
};

#[tokio::main]
async fn main() {
    let code = match run().await {
        Ok(()) => 0,
        Err(error) => {
            eprintln!("{error}");
            1
        }
    };
    // ServiceApp owns graceful shutdown. Runtime destruction must not wait
    // indefinitely for detached blocking work after its deadline has expired.
    std::process::exit(code);
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (config_path, values_path) =
        config_paths("./config/config.yaml", "./config/overrides.yaml");
    let noop_logs = environment_flag_enabled("SERVICELIB_NOOP_LOGS");
    let noop_metrics = environment_flag_enabled("SERVICELIB_NOOP_METRICS");
    let noop_tracing = environment_flag_enabled("SERVICELIB_NOOP_TRACING");
    let telemetry = if environment_flag_enabled("SERVICELIB_OTEL_ENABLED")
        && !(noop_logs && noop_metrics && noop_tracing)
    {
        Some(OpenTelemetry::install(
            OpenTelemetryConfig::from_environment("Inventory Service"),
        )?)
    } else {
        install_stdout(!noop_logs, !noop_tracing)?;
        None
    };
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
    let environment = if noop_tracing {
        environment.without_tracing()
    } else {
        environment
    };
    environment.publish_runtime_config(loader.runtime_config());
    install_shutdown_deadline(loader.clone())?;
    Service::new(&config, environment, loader).await?.run().await
}

fn install_shutdown_deadline(loader: ConfigLoader<Config>) -> std::io::Result<()> {
    let (ready, initialized) = std::sync::mpsc::sync_channel(1);
    std::thread::Builder::new()
        .name("shutdown-deadline".to_owned())
        .spawn(move || {
            // Signals and the deadline must progress even if every application
            // worker is blocked. No extra crate or application executor is used.
            let wait_for_signal = || -> std::io::Result<()> {
                let runtime = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()?;
                runtime.block_on(async {
                    #[cfg(unix)]
                    {
                        use tokio::signal::unix::{SignalKind, signal};
                        let mut interrupt = signal(SignalKind::interrupt())?;
                        let mut terminate = signal(SignalKind::terminate())?;
                        let _ = ready.send(Ok(()));
                        tokio::select! {
                            _ = interrupt.recv() => {},
                            _ = terminate.recv() => {},
                        }
                    }
                    #[cfg(windows)]
                    {
                        let mut interrupt = tokio::signal::windows::ctrl_c()?;
                        let _ = ready.send(Ok(()));
                        interrupt.recv().await;
                    }
                    Ok(())
                })
            };
            if let Err(error) = wait_for_signal() {
                let _ = ready.send(Err(error));
                return;
            }
            let started = std::time::Instant::now();
            let timeout = std::time::Duration::from_millis(
                loader.current().service().shutdown_timeout.max(0) as u64,
            );
            std::thread::sleep(timeout.saturating_sub(started.elapsed()));
            // Normal signal shutdown has the same exit status as graceful exit.
            // This policy belongs to the executable, never to library Stop.
            std::process::exit(0);
        })?;
    initialized.recv().map_err(std::io::Error::other)?
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