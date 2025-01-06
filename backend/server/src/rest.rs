// Here the settings for the REST API server will be made

use std::net::SocketAddr;

use axum::Router;
use common::types::websockets::WebsocketMessageBody;
use owo_colors::colors::css::*;
use owo_colors::OwoColorize;
use sqlx::{Pool, Sqlite};
use tracing::debug;

use super::create_routes::create_routes;

#[derive(Clone, Debug)]
struct ConnectionSettings {
    host_ip_address: [u8; 4],
    http: u16,
}

pub async fn start_http_server(db_client: Pool<Sqlite>, ws_body_channel_sender: tokio::sync::broadcast::Sender<WebsocketMessageBody>) {
    let router: Router = create_routes(db_client, ws_body_channel_sender);

    let port_http: u16 = std::env::var("PORT_RS_HTTP")
        .unwrap_or_else(|_| "4000".to_string())
        .parse()
        .expect("PORT_RS_HTTP must be a valid u16");

    let connection_settings = ConnectionSettings {
        host_ip_address: [0, 0, 0, 0],
        http: port_http,
    };

    let addr = SocketAddr::from((
        connection_settings.host_ip_address,
        connection_settings.http,
    ));
    let tcp_listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();

    let host_trace_address: String = connection_settings
        .host_ip_address
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(":");

    debug!(
        "{} {} {} {}:{}",
        "Members".fg::<Yellow>(),
        "Server".fg::<Yellow>(),
        "listening @",
        host_trace_address,
        connection_settings.http
    );

    axum::serve(
        tcp_listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
