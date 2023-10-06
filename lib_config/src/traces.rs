use crate::environment::{EnvironmentVariables, DEV_ENV};
use std::sync::Once;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

static TRACER_INITIALIZED: Once = Once::new();
use tracing::Subscriber;


pub fn setup_tracing_level(env: &EnvironmentVariables) {
    TRACER_INITIALIZED.call_once(|| {
        // Set the logging level based on the trace_level environment variable
        let level = match env.trace_level() {
            Some(l) if l == "info" => Level::INFO,
            _ => Level::ERROR,
        };

        let subscriber: Box<dyn Subscriber + Send + Sync> = if let Some(environment) = env.environment() {
            if environment == DEV_ENV {
                Box::new(FmtSubscriber::builder()
                    .with_max_level(level)
                    .with_ansi(true)
                    .with_target(true)
                    .finish())
            } else {
                Box::new(FmtSubscriber::builder()
                    .with_max_level(level)
                    .with_ansi(false)
                    .without_time()
                    .finish())
            }
        } else {
            Box::new(FmtSubscriber::builder()
                .with_max_level(level)
                .finish())
        };

        tracing::subscriber::set_global_default(subscriber)
            .expect("Setting default subscriber failed");

        tracing::info!("Tracing initialized");
    });
}

