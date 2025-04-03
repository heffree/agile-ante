use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::{AppError, Application};

pub fn get_room_routes() -> Router<Application> {
    Router::new()
        .route("/create-room", post(create_room_handler))
        .route("/get-rooms", get(get_rooms_handler))
        .route("/get-players", get(get_players_handler))
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

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct GetPlayersResponse {
    pub players: Vec<Uuid>,
}

async fn get_players_handler(
    State(state): State<Application>,
) -> Result<Json<GetPlayersResponse>, AppError> {
    let players = state.room_repo.get_rooms().await?;

    Ok(Json(GetPlayersResponse { players }))
}
