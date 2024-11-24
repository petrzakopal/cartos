use reader::core::connect::initialize_readers;
use tracing::{debug, info};
use tracing_log::LogTracer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    LogTracer::init().expect("Failed to set the log tracer.");

    let log_subscriber = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()) // use RUST_LOG for level
        .finish();

    if let Err(err) = tracing::subscriber::set_global_default(log_subscriber) {
        eprintln!("Failed to set the global subscriber: {:?}.", err);
    }

    info!("Starting the application.");

    debug!("Initialize the readers.");

    initialize_readers();
}
