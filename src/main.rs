mod api;
use crate::api::models::globals::commands::*;
use crate::api::repository::gpio::gpio_repository::{
    set_start,
    set_turnside, set_stop
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

use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

fn bit_handler(bytes: MutexGuard<'_, Vec<u8>>) {
    println!("{:?}", bytes);
    unsafe {
        match bytes.first() {
            Some(&TURN_LEFT) => set_turnside(true),
            Some(&TURN_RIGHT) => set_turnside(false),
            Some(&MOVE_FORWARD) =>  set_start(true),
            Some(&MOVE_BACKWARD) => set_start(false),
            Some(&STOP) => set_stop(),

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
        .route("/ping", get(ping))
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

async fn ping () {
    println!("hello");
}

async fn handle_connection (mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message (msg).is_break() {
                return;
            }
        } else {
            tracing::info!("Client abruptly disconnected");
            return;
        }
    }
}

fn process_message (msg: Message) -> ControlFlow<(), ()> {
    match msg {
        Message::Binary(array) => {
            let bytes = Arc::new(Mutex::new(array));
            let bytes = Arc::clone(&bytes);

            thread::spawn(move || bit_handler(bytes.lock().unwrap()));
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
            return ControlFlow::Break(());
        }
        _ => {
            tracing::info!("Sent not a binary info");
        }
    }

    ControlFlow::Continue(())
}
