use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    application::{AppError, Application},
    domain::player::Player,
};

pub fn get_room_routes() -> Router<Application> {
    Router::new().nest(
        "/rooms",
        Router::new()
            .route("/", post(create_room_handler))
            .route("/", get(get_rooms_handler))
            .route("/{id}/players", get(get_players_handler)),
    )
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
    pub players: Vec<PlayerJsonResponse>,
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct PlayerJsonResponse {
    pub id: usize,
    pub name: String,
}

async fn get_players_handler(
    State(state): State<Application>,
    Path(id): Path<String>,
) -> Result<Json<GetPlayersResponse>, AppError> {
    let players = state.rooms.get(&id).unwrap().lock().await;

    Ok(Json(GetPlayersResponse {
        players: Player::vec_to_get_json_response(players.get_players()),
    }))
}
