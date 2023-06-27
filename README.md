# Pomotodo

A simple pomodoro run in the command line, written in Rust. This is my first Rust project, if there is anything look wrong, please let me know. Appreciate your help!

## Support:

    - desktop notification

    - hacky notification sound with `rodio`

## Usage: 
    - `cargo run -- -c 4 -d 25 -s 5 -l 15` (4 pomodoros, 25 minutes each, 5 minutes short break, 15 minutes long break, which is default)

    - run `cargo run -- --help` for help

    - `count` is integer, `duration`, `short-break`, and `long-break` are all floating point number

## TODOS:

    - [ ] load config file

    - [ ] refactor `play_sound`

    - [ ] build binary and distribute?

    - [ ] UI with `tui-rs` (long term)
