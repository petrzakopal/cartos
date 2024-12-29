use common::{types::channels::{CardDataBroadcastChannel, CardData}, utils::handle_tokio_result::handle_task_result};
use db::{connection::user_validation, initialize_db::initialize_db};
use reader::core::connect::{read_loop};
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

    debug!("Initialize the database.");

    initialize_db().await;

    // Create broadcast channel for sending messages from read_loop to the database connection
    let (card_data_channel_sender, card_data_channel_receiver) = tokio::sync::broadcast::channel::<CardData>(300);
    let card_data_broadcast_channel : CardDataBroadcastChannel = CardDataBroadcastChannel {
        tx: card_data_channel_sender,
        rx: card_data_channel_receiver
    };


    // Reading card for ID or data
    let read_loop_handle = tokio::spawn(read_loop(card_data_broadcast_channel.tx.clone()));

    // Reaching to database for validation of users
    // Also log entries to database
    let database_connection_handle = tokio::spawn(user_validation(card_data_broadcast_channel.tx));


    // Join threads
    let (read_loop_tokjoin, database_connection_tokjion) = tokio::join!(read_loop_handle, database_connection_handle);

    // Handle the results gracefully
    handle_task_result(read_loop_tokjoin, "read_loop");
    handle_task_result(database_connection_tokjion, "read_loop");


}
