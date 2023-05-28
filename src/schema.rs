use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Hotkey {
    pub key: String,
    pub commands: Vec<Command>,
    pub loop_delay: Option<u64>,
}

#[derive(Deserialize)]
pub struct Config {
    pub hotkeys: HashMap<String, Hotkey>,
}

#[derive(Deserialize)]
pub struct Command {
    pub mouse_cmd: String,
    pub modifier: String,
    pub position_type: String,
    pub position_coords: (i32, i32),
}
