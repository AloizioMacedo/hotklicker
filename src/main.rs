pub mod parsed_schema;
pub mod schema;

use std::collections::HashMap;
use std::{panic::catch_unwind, time::Duration};

use enigo::Key as EniKey;
use enigo::MouseButton as EniMouse;
use enigo::{KeyboardControllable, MouseControllable};
use inputbot::KeybdKey as InpKey;
use schema::Config;

use parsed_schema::{ParsedCommand, ParsedConfig, ParsedHotkey, PositionType};

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
            let parsed_mouse: EniMouse =
                EniMouse::from_str(&command.mouse_cmd).expect("Should be parseable");
            let parsed_modifier: EniKey =
                EniKey::from_str(&command.modifier).expect("Should be parseable.");
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
        };

        parsed_config.insert(hotkey_name, parsed_hotkey);
    }

    ParsedConfig {
        hotkeys: parsed_config,
    }
}

fn get_closure_from_commands(commands: &[ParsedCommand], name: String) -> impl Fn() -> () {
    let modifiers: Vec<EniKey> = commands.iter().map(|&x| x.modifier).collect();
    let clicks: Vec<EniMouse> = commands.iter().map(|&x| x.mouse_command).collect();
    let positions: Vec<(i32, i32)> = commands.iter().map(|&x| x.position_coords).collect();
    let position_types: Vec<PositionType> = commands.iter().map(|&x| x.position_type).collect();

    move || {
        let mut enigo = enigo::Enigo::new();

        let current_location = enigo.mouse_location();

        for (((modifier, click), position), position_type) in modifiers
            .iter()
            .zip(clicks.iter())
            .zip(positions.iter())
            .zip(position_types.iter())
        {
            println!("Running command {}.", name);

            match position_type {
                PositionType::Absolute => enigo.mouse_move_to(position.0, position.1),
                PositionType::Relative => enigo.mouse_move_relative(position.0, position.1),
            }

            enigo.key_down(*modifier);
            enigo.mouse_click(*click);
            enigo.key_up(*modifier);

            std::thread::sleep(Duration::from_millis(50));
        }

        enigo.mouse_move_to(current_location.0, current_location.1)
    }
}

fn main() {
    let config = get_config();
    let parsed_config = parse_config(config);

    for (name, hotkey) in parsed_config.hotkeys {
        let commands = hotkey.commands;

        let cl = get_closure_from_commands(&commands, name);

        hotkey.key.bind(cl);
    }

    // InpKey::LKey.bind(|| loot_below());
    // InpKey::CapsLockKey.bind(|| loot_around());

    loop {
        let x = catch_unwind(|| inputbot::handle_input_events());

        if let Err(_) = x {
            println!("Could not identify key.")
        }
    }
}

fn _loot_below() {
    let mut enigo = enigo::Enigo::new();
    let current_location = enigo.mouse_location();

    enigo.mouse_move_to(870, 490);
    enigo.key_down(EniKey::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);

    let (x, y) = current_location;
    enigo.mouse_move_to(x, y);
    enigo.key_up(EniKey::Alt);
}

fn _loot_around() {
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
