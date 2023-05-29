# Introduction

Hotklicker is a software made in Rust that allows for setting hotkeys that will
execute actions with the mouse.

Actions can be defined with great flexibility as a sequence of commands that
move the mouse and can click with modifier keys.

An example of how to set up the hotkeys can be seen in this project's
config.yml.


OBS: If a hotkey has a loop_delay attribute, the only way to end the loop with
the current implementation is by pressing ESC, **not** the same key that sets up
the hotkey. This will kill the process and you must rerun it if you want
to use another hotkey.
