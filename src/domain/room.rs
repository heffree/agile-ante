use std::str::FromStr;

use tokio::sync::broadcast;
use uuid::Uuid;

use super::{player::Player, poker_event::PokerEvent};

#[derive(Clone)]
pub struct Room {
    pub broadcaster: broadcast::Sender<PokerEvent>,
    players: Vec<Player>,
}

impl Room {
    pub fn new(broadcaster: broadcast::Sender<PokerEvent>) -> Room {
        Room {
            broadcaster,
            players: vec![],
        }
    }

    pub fn add_player(&mut self, new_player: Player) {
        if !self
            .players
            .iter()
            .find(|player| player.id == new_player.id)
            .is_some()
        {
            self.players.push(new_player);
        }
    }

    pub fn get_players(&self) -> Vec<&Player> {
        self.players.iter().collect()
    }

    pub fn remove_player(&mut self, device_id: &str) {
        let uuid = Uuid::from_str(&device_id).unwrap();
        self.players.retain(|player| player.device_id != uuid);
    }

    pub fn get_player_count(&self) -> usize {
        self.players.len()
    }
}
