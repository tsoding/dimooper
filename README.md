[![Build Status](https://travis-ci.org/tsoding/dimooper.svg?branch=master)](https://travis-ci.org/tsoding/dimooper)

# dimooper #

Digital Music Looper application focused on live performance.

![screenshot01](http://i.imgur.com/zviXXXm.png)
![screenshot02](http://i.imgur.com/eedQ7d2.png)

## Building ##

### Linux ###

Prerequisite:
- Rust 1.8.0+
- Cargo 0.9.0+
- libportmidi
- SDL2

Having installed all of that just run `$ cargo run`

### NixOS ###

    $ nix-shell dimooper.nix -A dimooperEnv
    $ cargo run
