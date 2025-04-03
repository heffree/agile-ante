use std::str::FromStr;

use tokio::sync::broadcast;
use uuid::Uuid;

use super::poker_event::PokerEvent;

#[derive(Clone)]
pub struct Room {
    pub broadcaster: broadcast::Sender<PokerEvent>,
    players: Vec<Uuid>,
}

impl Room {
    pub fn new(broadcaster: broadcast::Sender<PokerEvent>) -> Room {
        Room {
            broadcaster,
            players: vec![],
        }
    }

    pub fn add_player(&mut self, id: &str) {
        let uuid = Uuid::from_str(&id).unwrap();
        self.players.push(uuid);
    }

    pub fn remove_player(&mut self, id: &str) {
        let uuid = Uuid::from_str(&id).unwrap();
        self.players.retain(|x| *x != uuid);
    }

    pub fn get_player_count(&self) -> usize {
        self.players.len()
    }
}
