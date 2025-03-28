use axum::{extract::State, routing::post, Json, Router};

use crate::domain::{app_state::AppState, poker_event::PokerEvent};

pub fn get_room_routes() -> Router<AppState> {
    Router::new().route("/increment", post(increment_handler))
}

async fn increment_handler(State(state): State<AppState>) -> Json<PokerEvent> {
    let room = &state.rooms[0];
    let mut count = room.count.lock().await;
    *count += 1;

    let event = PokerEvent {
        command: "new_count".into(),
        value: *count,
    };

    let _ = room.broadcaster.send(event.clone());

    Json(event)
}
