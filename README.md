[![Build Status](https://travis-ci.org/tsoding/dimooper.svg?branch=master)](https://travis-ci.org/tsoding/dimooper)

# dimooper #

Digital Music Looper application focused on live performance.

![screenshot](http://i.imgur.com/S5YzYiR.png)

## Demo ##

[![DIMOOPER DEMO](https://img.youtube.com/vi/qURmwdedUAI/0.jpg)](https://www.youtube.com/watch?v=qURmwdedUAI)

## Quick Start ##

```console
$ cargo build                               # build dimooper
$ cargo run <input-port> <output-port>  # run dimooper
$ cargo test                                # run unit tests
```

## Coverage ##

```console
$ cargo install kcov
$ cargo build
$ cargo kcov
$ <browser> target/cov/index.html
```

<!-- TODO(#222): document NixOS environment setup -->

## Setting Up With a Controller ##

Please, take a look at https://github.com/tsoding/dimooper/wiki/NixOS-QSynth-Setup-Guide
