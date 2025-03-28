use serde::Serialize;

#[derive(Serialize, Clone, Default)]
pub struct PokerEvent {
    pub command: String,
    pub value: usize,
}
