
use common::{
    hw::gpio::gpio_set_all_to_low,
    types::{channels::{CardData, CardDataBroadcastChannel, WebsocketBodyBroadcastChannel}, websockets::WebsocketMessageBody},
    utils::{
        handle_tokio_result::handle_task_result, load_env::load_env, perform_reset_with_usb_unplug,
    },
};
use db::{
    connection::{get_sqlite_db_pool, user_validation},
    initialize_db::{initialize_db, run_migrations_sqlite},
};
use reader::core::connect::read_loop;
use server::rest::start_http_server;
use tracing::{debug, info};
use tracing_log::LogTracer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    sudo::escalate_if_needed().expect("Could not run as a sudo.");
    LogTracer::init().expect("Failed to set the log tracer.");

    let log_subscriber = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()) // use RUST_LOG for level
        .finish();

    if let Err(err) = tracing::subscriber::set_global_default(log_subscriber) {
        eprintln!("Failed to set the global subscriber: {:?}.", err);
    }

    info!("Starting the application.");

    debug!("Set all GPIO to LOW before operation.");
    gpio_set_all_to_low();

    debug!("Initialize the database.");

    load_env();

    initialize_db().await;
    run_migrations_sqlite().await;

    perform_reset_with_usb_unplug::perform_reset_with_nfc_usb_unplug().await;

    // Create broadcast channel for sending messages from read_loop to the database connection
    let (card_data_channel_sender, card_data_channel_receiver) =
        tokio::sync::broadcast::channel::<CardData>(300);
    let card_data_broadcast_channel: CardDataBroadcastChannel = CardDataBroadcastChannel {
        tx: card_data_channel_sender,
        rx: card_data_channel_receiver,
    };

    // Create broadcast channel for sending websocket messages
    let (websocket_message_channel_sender, websocket_message_channel_receiver) =
        tokio::sync::broadcast::channel::<WebsocketMessageBody>(300);
    let ws_message_broadcast_channel: WebsocketBodyBroadcastChannel = WebsocketBodyBroadcastChannel {
        tx: websocket_message_channel_sender,
        rx: websocket_message_channel_receiver,
    };

    // Reading card for ID or data
    let read_loop_handle = tokio::spawn(read_loop(card_data_broadcast_channel.tx.clone()));

    // Reaching to database for validation of users
    // Also log entries to database
    let database_connection_handle = tokio::spawn(user_validation(card_data_broadcast_channel.tx, ws_message_broadcast_channel.tx.clone()));

    let http_server_handle = tokio::spawn(start_http_server(get_sqlite_db_pool().await, ws_message_broadcast_channel.tx.clone()));


    // Join threads
    let (read_loop_tokjoin, database_connection_tokjoin, http_server_tokjoin) = tokio::join!(
        read_loop_handle,
        database_connection_handle,
        http_server_handle
    );

    // wil try to incorporate another async function spawned by the tokio
    // which listens to the ctrl+c command and kills all the child processes and itself too

    // Handle the results gracefully
    handle_task_result(read_loop_tokjoin, "read_loop");
    handle_task_result(database_connection_tokjoin, "read_loop");
    handle_task_result(http_server_tokjoin, "http_server");
}

