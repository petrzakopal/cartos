// Here the REST API routes will be set

use std::sync::Arc;

use axum::{routing::{any, get, post}, Router};

use sqlx::{Pool, Sqlite};

use crate::{routes::log::get_all::get_all_logs, websocket::ws_handler, websocket_clients::Clients};

// Shared state in the AXUM routes
#[derive(Clone)]
pub struct AppState {
    pub db_sqlite_pool: Pool<Sqlite>, // MongoDB database client
    pub clients: Clients,
}

pub fn create_routes(db_sqlite_pool: Pool<Sqlite>) -> Router {
    // Init the state used in routes
    let app_state = Arc::new(AppState {
        db_sqlite_pool: db_sqlite_pool,
        clients: Clients::new(),
    });

    let app: Router = Router::new()
        .route("/user/add", post({}))
        .route("/user/view/all", post({}))
        .route("/user/view/single/email", post({}))
        .route("/user/view/single/card_serial_number", post({}))
        .route("/log/get/all", post(get_all_logs))
        .route("/log/get/single/email", post({}))
        .route("/log/get/single/card_serial_number", post({}))
        .route("/ws", get(ws_handler))
        // maybe add .with_state and AppState and channels for web sockets
        .layer(
            tower::ServiceBuilder::new()
                .layer(tower_http::trace::TraceLayer::new_for_http())
                .layer(tower_http::cors::CorsLayer::permissive()),
        )
        .with_state(app_state);

    return app;
}

