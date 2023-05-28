use std::{panic::catch_unwind, time::Duration};

use enigo::{KeyboardControllable, MouseControllable};

fn main() {
    inputbot::KeybdKey::LKey.bind(|| loot_below());
    inputbot::KeybdKey::CapsLockKey.bind(|| loot_around());

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
    enigo.key_down(enigo::Key::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);

    let (x, y) = current_location;
    enigo.mouse_move_to(x, y);
    enigo.key_up(enigo::Key::Alt);
}

fn loot_around() {
    let mut enigo = enigo::Enigo::new();
    let current_location = enigo.mouse_location();

    let delay = Duration::from_millis(43);

    // Center.
    enigo.mouse_move_to(870, 490);
    enigo.key_down(enigo::Key::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);
    enigo.key_up(enigo::Key::Alt);
    std::thread::sleep(delay);

    // N.
    enigo.mouse_move_relative(0, -70);
    enigo.key_down(enigo::Key::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);
    enigo.key_up(enigo::Key::Alt);
    std::thread::sleep(delay);

    // NE.
    enigo.mouse_move_relative(70, 0);
    enigo.key_down(enigo::Key::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);
    enigo.key_up(enigo::Key::Alt);
    std::thread::sleep(delay);

    // E.
    enigo.mouse_move_relative(0, 70);
    enigo.key_down(enigo::Key::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);
    enigo.key_up(enigo::Key::Alt);
    std::thread::sleep(delay);

    // SE.
    enigo.mouse_move_relative(0, 70);
    enigo.key_down(enigo::Key::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);
    enigo.key_up(enigo::Key::Alt);
    std::thread::sleep(delay);

    // S.
    enigo.mouse_move_relative(-70, 0);
    enigo.key_down(enigo::Key::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);
    enigo.key_up(enigo::Key::Alt);
    std::thread::sleep(delay);

    // SW.
    enigo.mouse_move_relative(-70, 0);
    enigo.key_down(enigo::Key::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);
    enigo.key_up(enigo::Key::Alt);
    std::thread::sleep(delay);

    // W.
    enigo.mouse_move_relative(0, -70);
    enigo.key_down(enigo::Key::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);
    enigo.key_up(enigo::Key::Alt);
    std::thread::sleep(delay);

    // NW.
    enigo.mouse_move_relative(0, -70);
    enigo.key_down(enigo::Key::Alt);
    enigo.mouse_click(enigo::MouseButton::Left);
    enigo.key_up(enigo::Key::Alt);
    std::thread::sleep(delay);

    let (x, y) = current_location;
    enigo.mouse_move_to(x, y);
}
