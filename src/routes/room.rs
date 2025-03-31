use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    application::{AppError, Application},
    domain::poker_event::PokerEvent,
};

pub fn get_room_routes() -> Router<Application> {
    Router::new()
        .route("/create-room", post(create_room_handler))
        .route("/get-rooms", get(get_rooms_handler))
        .route("/increment", post(increment_handler))
}

/// Used to direct the room creator to their room.
#[derive(Serialize, Clone, Default)]
struct NewRoomResponse {
    room_id: Uuid,
}

async fn create_room_handler(
    State(state): State<Application>,
) -> Result<Json<NewRoomResponse>, AppError> {
    let room_id = state.room_repo.create_new().await?;

    Ok(Json(NewRoomResponse { room_id }))
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct GetRoomsResponse {
    pub room_ids: Vec<Uuid>,
}

async fn get_rooms_handler(
    State(state): State<Application>,
) -> Result<Json<GetRoomsResponse>, AppError> {
    let room_ids = state.room_repo.get_rooms().await?;

    Ok(Json(GetRoomsResponse { room_ids }))
}

async fn increment_handler(State(state): State<Application>) -> Json<PokerEvent> {
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
