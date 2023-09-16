mod api;
use crate::api::models::globals::commands::*;
use crate::api::repository::gpio::gpio_repository::{
    set_start,
    set_turnside, set_execution
};

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
  };
use tower_http::trace::{TraceLayer, DefaultMakeSpan};

use std::{
    net::SocketAddr,
    ops::ControlFlow
};


use std::sync::Arc;
use std::thread;

fn bit_handler(bytes: Arc<Vec<u8>>) {
    unsafe {
        match bytes.first() {
            Some(&TURN) => set_turnside(bytes),
            Some(&MOVE_FORWARD) =>  set_start(true),
            Some(&MOVE_BACKWARD) => set_start(false),

            _ => tracing::info!("Sent unknown command")
        }
    }
}
  
#[tokio::main]
async fn main () {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("WebSocket started");

    let app = Router::new()
        .route("/websocket", get(ws_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true))
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
  
async fn ws_handler (
    ws: WebSocketUpgrade
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_connection(socket))
}

async fn handle_connection (mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message (msg).is_break() {
                unsafe { set_execution() };
                return;
            }

            if socket
                .send(Message::Binary(vec![0x00]))
                .await
                .is_err()
            {
                println!("client abruptly disconnected");
                unsafe { set_execution() };
                return;
            }
        } else {
            tracing::info!("Client abruptly disconnected");
            unsafe { set_execution() };
            return;
        }
    }
}

fn process_message (msg: Message) -> ControlFlow<(), ()> {
    match msg {
        Message::Binary(array) => {
            let bytes = Arc::new(array);
            let bytes = Arc::clone(&bytes);

            thread::spawn(move || bit_handler(bytes));
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                tracing::info!(
                    "Sent close with code {} and reason `{}`",
                    cf.code, cf.reason
                );
            } else {
                tracing::info!("Sent close message without CloseFrame");
            }

            unsafe { set_execution() };
            return ControlFlow::Break(());
        }
        _ => {
            tracing::info!("Sent not a binary info");
        }
    }

    ControlFlow::Continue(())
}
