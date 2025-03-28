use std::sync::Arc;

use tokio::sync::{broadcast, Mutex};

use super::poker_event::PokerEvent;

#[derive(Clone)]
pub struct RoomState {
    pub count: Arc<Mutex<usize>>,
    pub broadcaster: broadcast::Sender<PokerEvent>,
}
