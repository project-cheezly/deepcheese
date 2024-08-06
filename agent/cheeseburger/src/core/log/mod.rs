use opentelemetry::{
    logs::LogError,
    KeyValue,
};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, Resource};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

mod config;

fn init_log_provider(cfg: &config::LogConfig)
    -> Result<opentelemetry_sdk::logs::LoggerProvider, LogError> 
{
    opentelemetry_otlp::new_pipeline()
        .logging()
        .with_resource(Resource::new(vec![KeyValue::new(
            opentelemetry_semantic_conventions::resource::SERVICE_NAME,
            "cheeseburger"
        )]))
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(cfg.host.clone()),
        )
        .install_batch(runtime::Tokio)
}

pub async fn init_log()
    -> Result<(), Box<dyn std::error::Error + Sync + Send>>
{
    let cfg = config::load().await?;
    
    let logger_provider = init_log_provider(&cfg)?;
    let layer = OpenTelemetryTracingBridge::new(&logger_provider);
    
    let filter = EnvFilter::new("info")
        .add_directive("hyper=error".parse().unwrap())
        .add_directive("tonic=error".parse().unwrap())
        .add_directive("reqwest=error".parse().unwrap());
    
    tracing_subscriber::registry()
        .with(filter)
        .with(layer)
        .init();
    
    Ok(())
}