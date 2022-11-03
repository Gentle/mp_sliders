use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ServerMessage {
    Sliders(usize),
    Update(Vec<(usize, u8)>),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ClientMessage {
    Add,
    Remove,
    Update(usize, u8),
}
