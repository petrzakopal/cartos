use std::{borrow::Cow, net::SocketAddr, ops::ControlFlow, sync::Arc};

use axum::{
    extract::{
        ws::{CloseFrame, Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use axum_extra::{headers, TypedHeader};
use common::utils::handle_tokio_result::handle_task_result;
use tracing::{error, info};

//allows to split the websocket stream into separate TX and RX branches
use futures::{
    lock::Mutex,
    sink::SinkExt,
    stream::{SplitSink, StreamExt},
};

use crate::create_routes::AppState;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(app_state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };

    info!("`{user_agent} at {addr} connected.`");

    return ws.on_upgrade(move |socket| handle_socket(socket, addr, State(app_state)));
}

async fn handle_socket(
    mut socket: WebSocket,
    who: SocketAddr,
    State(app_state): State<Arc<AppState>>,
) {
    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (sender, mut receiver) = socket.split();

    let sender = Arc::new(Mutex::new(sender));
    // Adding client to a hash map
    app_state.clients.add(sender.clone()).await;
    let send_task_state = Arc::clone(&app_state);

    // Spawn a task that will push several messages to the client (does not matter what client does)
    let mut send_task = tokio::spawn({
        let sender = Arc::clone(&sender);
        async move {
            if sender
                .lock()
                .await
                .send(Message::Text(
                    format!("Hello from websockets send_task").into(),
                ))
                .await
                .is_err()
            {
                error!("Error sending message in send task.");
            }
        }
        // Trying to send some message
    });

    // Receiving messages from clients
    let mut recv_task = tokio::spawn({
        let sender = Arc::clone(&sender);
        async move {
            let mut cnt = 0;
            while let Some(Ok(msg)) = receiver.next().await {
                cnt += 1;
                // print message and break if instructed to do so
                if process_message(msg, who, &sender, &send_task_state)
                    .await
                    .is_break()
                {
                    break;
                }
            }
            cnt
        }
    });

    let (send_task_tokjoin, recv_task_tokjoin) = tokio::join!(send_task, recv_task);
    handle_task_result(send_task_tokjoin, "send_task");
    handle_task_result(recv_task_tokjoin, "rect_task");
}

async fn process_message(
    msg: Message,
    who: SocketAddr,
    sender_mt: &Arc<Mutex<SplitSink<WebSocket, Message>>>,
    app_state: &Arc<AppState>,
) -> ControlFlow<(), ()> {
    let sender = sender_mt.clone();
    match msg {
        Message::Text(t) => {
            println!(">>> {who} sent str: {t:?}");
            // this broadcasts the message to all connected clients
            app_state
                .clients
                .broadcast(Message::Text(format!(
                    "{}",
                    t
                )))
                .await;

            // this sends the message as an echo only to the sender
            if sender
                .lock()
                .await
                .send(Message::Text(
                    //format!("Hello from websockets recv_task, you have sent {}", t).into(),
                    format!("{}", t).into(),
                ))
                .await
                .is_err()
            {
                error!("Error sending message in send task.");
            }
        }
        Message::Binary(d) => {
            println!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                println!(">>> {who} somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            println!(">>> {who} sent pong with {v:?}");
        }
        Message::Ping(v) => {
            println!(">>> {who} sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}
