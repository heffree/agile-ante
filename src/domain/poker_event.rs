use serde::Serialize;

/// `PokerEvent` exists so that we can send state changes to the FE incrementally.
/// It should eventually include some sort logical number so each client can determine
/// if they've dropped a message and regrab latest.
#[derive(Serialize, Clone)]
#[serde(tag = "command", content = "value")]
pub enum PokerEvent {
    PlayerJoined { id: String },
    PlayerLeft { id: String },
}
