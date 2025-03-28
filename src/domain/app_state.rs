use super::room_state::RoomState;

#[derive(Clone)]
pub struct AppState {
    pub rooms: Vec<RoomState>,
}
