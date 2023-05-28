use std::collections::HashMap;
use std::{panic::catch_unwind, time::Duration};

use enigo::Key as EniKey;
use enigo::{KeyboardControllable, MouseControllable};
use inputbot::KeybdKey as InpKey;
use serde::{Deserialize, Serialize};

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
            _ => Err(()),
        }
    }
}

impl FromStr<EniKey> for EniKey {
    fn from_str(s: &str) -> Result<EniKey, ()> {
        match s {
            _ => Err(()),
        }
    }
}

#[derive(Deserialize)]
struct Command {
    key: String,
    modifier: String,
    position_type: String,
    position_coords: (u16, u16),
}

enum PositionType {
    Absolute,
    Relative,
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

struct ParsedCommand {
    key: EniKey,
    modifier: EniKey,
    position_type: PositionType,
    position_coords: (u16, u16),
}

#[derive(Deserialize)]
struct Hotkey {
    key: String,
    commands: Vec<Command>,
}

struct ParsedHotkey {
    key: InpKey,
    commands: Vec<ParsedCommand>,
}

#[derive(Deserialize)]
struct Config {
    hotkeys: HashMap<String, Hotkey>,
}

struct ParsedConfig {
    hotkeys: HashMap<String, ParsedHotkey>,
}

fn get_config() -> Config {
    let file = std::fs::File::open(FILE).expect("config.yml file should be present");

    serde_yaml::from_reader(file).expect("config.yml file not properly formatted.")
}

fn parse_config(config: Config) -> ParsedConfig {
    let mut parsed_config = HashMap::new();

    for (hotkey_name, hotkey) in config.hotkeys {
        let mut parsed_commands = Vec::new();
        let commands = hotkey.commands;

        for command in commands {
            let parsed_key: EniKey = EniKey::from_str(&command.key).expect("Should be parseable");
            let parsed_modifier: EniKey =
                EniKey::from_str(&command.key).expect("Should be parseable.");
            let position_type =
                PositionType::from_str(&command.position_type).expect("Should be parseable");

            parsed_commands.push(ParsedCommand {
                key: parsed_key,
                modifier: parsed_modifier,
                position_type,
                position_coords: command.position_coords,
            })
        }

        let parsed_hotkey = ParsedHotkey {
            key: InpKey::from_str(&hotkey.key).expect("Should be parseable"),
            commands: parsed_commands,
        };

        parsed_config.insert(hotkey_name, parsed_hotkey);
    }

    ParsedConfig {
        hotkeys: parsed_config,
    }
}

fn main() {
    InpKey::LKey.bind(|| loot_below());
    InpKey::CapsLockKey.bind(|| loot_around());

    loop {
        let x = catch_unwind(|| inputbot::handle_input_events());

        if let Err(_) = x {
            println!("Could not identify key.")
        }
    }
}

fn loot_below() {
    let mut enigo = enigo::Enigo::new();
    let current_location = enigo.mouse_location();

    enigo.mouse_move_to(870, 490);
    enigo.key_down(EniKey::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);

    let (x, y) = current_location;
    enigo.mouse_move_to(x, y);
    enigo.key_up(EniKey::Alt);
}

fn loot_around() {
    let mut enigo = enigo::Enigo::new();
    let current_location = enigo.mouse_location();

    let delay = Duration::from_millis(43);

    // Center.
    enigo.mouse_move_to(870, 490);
    enigo.key_down(EniKey::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);
    enigo.key_up(EniKey::Alt);
    std::thread::sleep(delay);

    // N.
    enigo.mouse_move_relative(0, -70);
    enigo.key_down(EniKey::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);
    enigo.key_up(EniKey::Alt);
    std::thread::sleep(delay);

    // NE.
    enigo.mouse_move_relative(70, 0);
    enigo.key_down(EniKey::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);
    enigo.key_up(EniKey::Alt);
    std::thread::sleep(delay);

    // E.
    enigo.mouse_move_relative(0, 70);
    enigo.key_down(EniKey::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);
    enigo.key_up(EniKey::Alt);
    std::thread::sleep(delay);

    // SE.
    enigo.mouse_move_relative(0, 70);
    enigo.key_down(EniKey::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);
    enigo.key_up(EniKey::Alt);
    std::thread::sleep(delay);

    // S.
    enigo.mouse_move_relative(-70, 0);
    enigo.key_down(EniKey::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);
    enigo.key_up(EniKey::Alt);
    std::thread::sleep(delay);

    // SW.
    enigo.mouse_move_relative(-70, 0);
    enigo.key_down(EniKey::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);
    enigo.key_up(EniKey::Alt);
    std::thread::sleep(delay);

    // W.
    enigo.mouse_move_relative(0, -70);
    enigo.key_down(EniKey::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);
    enigo.key_up(EniKey::Alt);
    std::thread::sleep(delay);

    // NW.
    enigo.mouse_move_relative(0, -70);
    enigo.key_down(EniKey::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);
    enigo.key_up(EniKey::Alt);
    std::thread::sleep(delay);

    let (x, y) = current_location;
    enigo.mouse_move_to(x, y);
}
