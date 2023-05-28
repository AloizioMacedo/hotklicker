use std::collections::HashMap;

use enigo::Key as EniKey;
use inputbot::KeybdKey as InpKey;

pub struct ParsedConfig {
    pub hotkeys: HashMap<String, ParsedHotkey>,
}

pub struct ParsedHotkey {
    pub key: InpKey,
    pub commands: Vec<ParsedCommand>,
}

pub struct ParsedCommand {
    pub key: EniKey,
    pub modifier: EniKey,
    pub position_type: PositionType,
    pub position_coords: (u16, u16),
}

pub enum PositionType {
    Absolute,
    Relative,
}
