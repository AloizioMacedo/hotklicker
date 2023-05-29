pub mod parsed_schema;
pub mod parsing;
pub mod schema;

use std::{panic::catch_unwind, time::Duration};

use enigo::{KeyboardControllable, MouseControllable};
use inputbot::KeybdKey as InpKey;
use parsing::{get_config, parse_config};

use parsed_schema::{ParsedCommand, PositionType};

fn main() {
    let config = get_config();
    let parsed_config = parse_config(config);

    for (name, hotkey) in &parsed_config.hotkeys {
        let commands = &hotkey.commands;

        let cl =
            get_closure_from_commands(commands, name.to_owned(), hotkey.loop_delay, hotkey.key);

        hotkey.key.bind(cl);
    }

    InpKey::EscapeKey.bind(|| std::process::exit(0));

    loop {
        let x = catch_unwind(|| inputbot::handle_input_events());

        if let Err(_) = x {
            println!("Could not identify key.");
        }
    }
}

fn get_closure_from_commands(
    commands: &[ParsedCommand],
    name: String,
    loop_delay: Option<u64>,
    hotkey: InpKey,
) -> impl Fn() -> () {
    let commands = commands.to_owned();

    move || {
        println!("Running command {}.", name);
        let mut enigo = enigo::Enigo::new();

        if let Some(sleep_time) = loop_delay {
            hotkey.bind(|| panic!("Loop interrupted."));

            loop {
                let current_location = enigo.mouse_location();

                run_commands(&commands, &mut enigo);

                enigo.mouse_move_to(current_location.0, current_location.1);
                std::thread::sleep(Duration::from_millis(sleep_time));
            }
        } else {
            let current_location = enigo.mouse_location();

            run_commands(&commands, &mut enigo);
            enigo.mouse_move_to(current_location.0, current_location.1);
        }
    }
}

fn run_commands(commands: &[ParsedCommand], enigo: &mut enigo::Enigo) {
    for command in commands {
        let pos_type = command.position_type;
        let pos = command.position_coords;

        match pos_type {
            PositionType::Absolute => enigo.mouse_move_to(pos.0, pos.1),
            PositionType::Relative => enigo.mouse_move_relative(pos.0, pos.1),
        }

        if let Some(modifier) = command.modifier {
            enigo.key_down(modifier);
            enigo.mouse_click(command.mouse_command);
            enigo.key_up(modifier);
        } else {
            enigo.mouse_click(command.mouse_command);
        }

        std::thread::sleep(Duration::from_millis(50));
    }
}
