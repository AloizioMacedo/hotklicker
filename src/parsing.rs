use crate::parsed_schema::{ParsedCommand, ParsedConfig, ParsedHotkey, PositionType};
use crate::schema::Config;

use enigo::Key as EniKey;
use enigo::MouseButton as EniMouse;
use inputbot::KeybdKey as InpKey;
use std::collections::HashMap;

const FILE: &str = "config.yml";

trait FromStr<T> {
    fn from_str(s: &str) -> Result<T, ()>;
}

impl FromStr<InpKey> for InpKey {
    fn from_str(s: &str) -> Result<InpKey, ()> {
        match s {
            "a" => Ok(InpKey::AKey),
            "b" => Ok(InpKey::BKey),
            "c" => Ok(InpKey::CKey),
            "d" => Ok(InpKey::DKey),
            "e" => Ok(InpKey::EKey),
            "f" => Ok(InpKey::FKey),
            "g" => Ok(InpKey::GKey),
            "h" => Ok(InpKey::HKey),
            "i" => Ok(InpKey::IKey),
            "j" => Ok(InpKey::JKey),
            "k" => Ok(InpKey::KKey),
            "l" => Ok(InpKey::LKey),
            "m" => Ok(InpKey::MKey),
            "n" => Ok(InpKey::NKey),
            "o" => Ok(InpKey::OKey),
            "p" => Ok(InpKey::PKey),
            "q" => Ok(InpKey::QKey),
            "r" => Ok(InpKey::RKey),
            "s" => Ok(InpKey::SKey),
            "t" => Ok(InpKey::TKey),
            "u" => Ok(InpKey::UKey),
            "v" => Ok(InpKey::VKey),
            "w" => Ok(InpKey::WKey),
            "x" => Ok(InpKey::XKey),
            "y" => Ok(InpKey::YKey),
            "z" => Ok(InpKey::ZKey),
            "caps lock" => Ok(InpKey::CapsLockKey),
            "tab" => Ok(InpKey::TabKey),
            "f1" => Ok(InpKey::F1Key),
            "f2" => Ok(InpKey::F2Key),
            "f3" => Ok(InpKey::F3Key),
            "f4" => Ok(InpKey::F4Key),
            "f5" => Ok(InpKey::F5Key),
            "f6" => Ok(InpKey::F6Key),
            "f7" => Ok(InpKey::F7Key),
            "f8" => Ok(InpKey::F8Key),
            "f9" => Ok(InpKey::F9Key),
            "f10" => Ok(InpKey::F10Key),
            "f11" => Ok(InpKey::F11Key),
            "f12" => Ok(InpKey::F12Key),
            _ => Err(()),
        }
    }
}

impl FromStr<EniKey> for EniKey {
    fn from_str(s: &str) -> Result<EniKey, ()> {
        let s = s.trim().to_lowercase();

        match s.as_str() {
            "alt" => Ok(EniKey::Alt),
            "ctrl" => Ok(EniKey::Control),
            "shift" => Ok(EniKey::Shift),
            _ => Err(()),
        }
    }
}

impl FromStr<EniMouse> for EniMouse {
    fn from_str(s: &str) -> Result<EniMouse, ()> {
        let s = s.trim().to_lowercase();

        match s.as_str() {
            "left click" => Ok(EniMouse::Left),
            "right click" => Ok(EniMouse::Right),
            "middle click" => Ok(EniMouse::Middle),
            _ => Err(()),
        }
    }
}

impl FromStr<PositionType> for PositionType {
    fn from_str(s: &str) -> Result<PositionType, ()> {
        let s = s.trim().to_lowercase();

        match s.as_str() {
            "absolute" => Ok(PositionType::Absolute),
            "relative" => Ok(PositionType::Relative),
            _ => Err(()),
        }
    }
}

pub fn get_config() -> Config {
    let file = std::fs::File::open(FILE).expect("config.yml file should be present");

    serde_yaml::from_reader(file).expect("config.yml file not properly formatted.")
}

pub fn parse_config(config: Config) -> ParsedConfig {
    let mut parsed_config = HashMap::new();

    for (hotkey_name, hotkey) in config.hotkeys {
        let mut parsed_commands = Vec::new();
        let commands = hotkey.commands;

        for command in commands {
            let parsed_mouse: EniMouse =
                EniMouse::from_str(&command.mouse_cmd).expect("Should be parseable");
            let parsed_modifier = command
                .modifier
                .map(|x| EniKey::from_str(&x).expect("Should be parseable"));
            let position_type =
                PositionType::from_str(&command.position_type).expect("Should be parseable");

            parsed_commands.push(ParsedCommand {
                mouse_command: parsed_mouse,
                modifier: parsed_modifier,
                position_type,
                position_coords: command.position_coords,
            })
        }

        let parsed_hotkey = ParsedHotkey {
            key: InpKey::from_str(&hotkey.key).expect("Should be parseable"),
            commands: parsed_commands,
            loop_delay: hotkey.loop_delay,
        };

        parsed_config.insert(hotkey_name, parsed_hotkey);
    }

    ParsedConfig {
        hotkeys: parsed_config,
    }
}
