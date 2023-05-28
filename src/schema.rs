use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Hotkey {
    pub key: String,
    pub commands: Vec<Command>,
}

#[derive(Deserialize)]
pub struct Config {
    pub hotkeys: HashMap<String, Hotkey>,
}

#[derive(Deserialize)]
pub struct Command {
    pub key: String,
    pub modifier: String,
    pub position_type: String,
    pub position_coords: (u16, u16),
}
