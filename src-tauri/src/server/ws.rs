use crate::state::AppState;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(app_state): State<AppState>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, app_state))
}

async fn handle_socket(socket: WebSocket, app_state: AppState) {
    let (mut sender, mut receiver) = socket.split();

    // Increment connected client count
    {
        let mut inner = app_state.inner.lock().await;
        inner.connected_clients += 1;
        let count = inner.connected_clients;
        let count_msg = json!({
            "type": "client_count",
            "data": { "count": count }
        })
        .to_string();
        let _ = app_state.broadcast_tx.send(count_msg);
    }

    let mut rx = app_state.broadcast_tx.subscribe();

    // Task for forwarding broadcast events to client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Task for receiving client messages (heartbeat/ping)
    let state_for_recv = app_state.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Close(_) => break,
                Message::Ping(_) => {
                    // Handled automatically by axum WebSocket
                }
                _ => {}
            }
        }
    });

    // Wait for either send or receive to end
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    // Decrement connected client count on disconnect
    {
        let mut inner = state_for_recv.inner.lock().await;
        if inner.connected_clients > 0 {
            inner.connected_clients -= 1;
        }
        let count = inner.connected_clients;
        let count_msg = json!({
            "type": "client_count",
            "data": { "count": count }
        })
        .to_string();
        let _ = state_for_recv.broadcast_tx.send(count_msg);
    }
}

