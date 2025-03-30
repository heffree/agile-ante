use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::{application::Application, domain::poker_event::PokerEvent};

pub fn get_room_routes() -> Router<Application> {
    Router::new()
        .route("/create-room", post(create_room_handler))
        .route("/get-rooms", get(get_rooms_handler))
        .route("/increment", post(increment_handler))
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = StatusCode::INTERNAL_SERVER_ERROR;
        (status, self.to_string()).into_response()
    }
}

/// Used to direct the room creator to their room.
#[derive(Serialize, Clone, Default)]
struct NewRoomResponse {
    room_id: Uuid,
}

async fn create_room_handler(
    State(state): State<Application>,
) -> Result<Json<NewRoomResponse>, AppError> {
    let room_id = state
        .room_repo
        .create_new()
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))?;
    Ok(Json(NewRoomResponse { room_id }))
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct GetRoomsResponse {
    pub room_ids: Vec<Uuid>,
}

async fn get_rooms_handler(
    State(state): State<Application>,
) -> Result<Json<GetRoomsResponse>, AppError> {
    let room_ids = state
        .room_repo
        .get_rooms()
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))?;

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
