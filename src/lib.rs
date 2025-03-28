use axum::{
    response::Html,
    routing::{get, Router},
};
use domain::{app_state::AppState, room_state::RoomState};
use routes::{client, room, sse};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::{broadcast, Mutex};

mod domain;
mod routes;

pub async fn start_server() {
    let (tx, _) = broadcast::channel(16);
    let state = AppState {
        rooms: vec![RoomState {
            count: Arc::new(Mutex::new(0)),
            broadcaster: tx,
        }],
    };

    let app = Router::new()
        .merge(sse::get_sse_routes())
        .merge(room::get_room_routes())
        .with_state(state)
        .merge(client::get_client_routes())
        .fallback_service(get(not_found));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn not_found() -> Html<&'static str> {
    Html("<h1>404</h1><p>Not Found</p>")
}
