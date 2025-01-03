// Here the REST API routes will be set

use std::sync::Arc;

use axum::{routing::{any, get, post}, Router};

use sqlx::{Pool, Sqlite};

use crate::{routes::{log::{view_all::view_all_logs, view_single_card_serial_number::view_single_log_by_card_serial_number, view_single_email::view_single_log_by_email}, system::restart_service::{self, do_restart_service}, user::{add::add_user, update_by_id::update_user_by_id, view_all::view_all_users, view_single_card_serial_number::view_single_user_by_card_serial_number, view_single_email::view_single_user_by_email}}, websocket::ws_handler, websocket_clients::Clients};

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
        .route("/system/restart", post(do_restart_service))
        .route("/user/update", post(update_user_by_id))
        .route("/user/add", post(add_user))
        .route("/user/view/all", post(view_all_users))
        .route("/user/view/single/email", post(view_single_user_by_email))
        .route("/user/view/single/card_serial_number", post(view_single_user_by_card_serial_number))
        .route("/log/view/all", post(view_all_logs))
        .route("/log/view/single/email", post(view_single_log_by_email))
        .route("/log/view/single/card_serial_number", post(view_single_log_by_card_serial_number))
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

