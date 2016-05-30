# Midi Looper #

Just Midi Looper application focused on live performance.

## Building ##

### Linux ###

Prerequisite:
- Rust 1.8.0+
- Cargo 0.9.0+
- libportmidi

Having installed all of that just run `$ cargo run`

### NixOS ###

    $ nix-shell midi-looper.nix -A midiLooperEnv
    $ cargo run
