use std::collections::HashMap;

use enigo::Key as EniKey;
use enigo::MouseButton as EniMouse;
use inputbot::KeybdKey as InpKey;

pub struct ParsedConfig {
    pub hotkeys: HashMap<String, ParsedHotkey>,
}

pub struct ParsedHotkey {
    pub key: InpKey,
    pub commands: Vec<ParsedCommand>,
    pub loop_delay: Option<u64>,
}

#[derive(Clone, Copy)]
pub struct ParsedCommand {
    pub mouse_command: EniMouse,
    pub modifier: EniKey,
    pub position_type: PositionType,
    pub position_coords: (i32, i32),
}

#[derive(Clone, Copy)]
pub enum PositionType {
    Absolute,
    Relative,
}
