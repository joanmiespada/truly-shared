// use opentelemetry::{global, KeyValue};

// use opentelemetry::sdk::propagation::TraceContextPropagator;
// use opentelemetry::sdk::{trace, Resource};
// use opentelemetry_otlp::WithExportConfig;
// use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
// use tracing_subscriber::Registry;
// use tracing_subscriber::{prelude::*, EnvFilter};

// use crate::config::Config;
// use crate::result::ResultE;

// pub fn init_telemetry(//exporter_endpoint: &str,
//     service_name: &str, conf: &Config) -> ResultE<()> {
//     match conf.env_vars().telemetry() {
//         None => {
//             println!("No telemetry initialized");
//             return Ok(());
//         }
//         Some(value) => {
//             if !value {
//                 println!("No telemetry initialized");
//                 return Ok(());
//             }
//         }
//     }
//     let exporter_endpoint = match conf.env_vars().telemetry_endpoint() {
//         None => panic!("telemetry enabled but endpoint is missing!!! add telemetry endpoint at env vars"),
//         Some(value) => value.to_string()
//     };

//     // Create a gRPC exporter
//     let exporter = opentelemetry_otlp::new_exporter()
//         .tonic()
//         .with_endpoint(   exporter_endpoint);

//     // Define a tracer
//     let tracer = opentelemetry_otlp::new_pipeline()
//         .tracing()
//         .with_exporter(exporter)
//         .with_trace_config(
//             trace::config().with_resource(Resource::new(vec![KeyValue::new(
//                 opentelemetry_semantic_conventions::resource::SERVICE_NAME,
//                 service_name.to_string(),
//             )])),
//         )
//         .install_batch(opentelemetry::runtime::Tokio)
//         .expect("Error: Failed to initialize the tracer.");

//     // Define a subscriber.
//     let subscriber = Registry::default();
//     // Level filter layer to filter traces based on level (trace, debug, info, warn, error).
//     let level_filter_layer = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("INFO"));
//     // Layer for adding our configured tracer.
//     let tracing_layer = tracing_opentelemetry::layer().with_tracer(tracer);
//     // Layer for printing spans to stdout
//     let formatting_layer = BunyanFormattingLayer::new(service_name.to_string(), std::io::stdout);
//     global::set_text_map_propagator(TraceContextPropagator::new());

//     subscriber
//         .with(level_filter_layer)
//         .with(tracing_layer)
//         .with(JsonStorageLayer)
//         .with(formatting_layer)
//         .init();

//     println!("Telemetry initialized successfully");
//     Ok(())
// }
