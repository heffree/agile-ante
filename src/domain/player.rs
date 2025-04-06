use uuid::Uuid;

use crate::routes::room::PlayerJsonResponse;

#[derive(Clone)]
pub struct Player {
    pub id: usize,
    pub device_id: Uuid, // Players act through their device_id, I guess
    pub player_name: String,
}

impl Player {
    // idk how to Into From right, I guess
    pub fn vec_to_get_json_response(players: Vec<&Player>) -> Vec<PlayerJsonResponse> {
        players
            .iter()
            .map(|p| PlayerJsonResponse {
                id: p.id,
                name: p.player_name.clone(),
            })
            .collect()
    }
}
