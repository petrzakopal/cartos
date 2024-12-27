use common::utils::handle_tokio_result::handle_task_result;
use reader::core::connect::{initialize_readers, read_loop};
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


    // Create broadcast channel for sending messages from read_loop to the database connection

    // Reading card for ID or data
    let read_loop_handle = tokio::spawn(read_loop());

    // Reaching to database for validation of users
    // Also log entries to database
    let database_connection_handle = tokio::spawn(async {});


    // Join threads
    let (read_loop_tokjoin, database_connection_tokjion) = tokio::join!(read_loop_handle, database_connection_handle);

    // Handle the results gracefully
    handle_task_result(read_loop_tokjoin, "read_loop");
    handle_task_result(database_connection_tokjion, "read_loop");


}
