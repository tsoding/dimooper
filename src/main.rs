// FIXME(#139): Deny warnings only on CI
// #![deny(warnings)]
// FIXME(#160): fix build with clippy
// #![feature(plugin)]
// #![plugin(clippy)]
#![feature(test)]

extern crate sdl2;
extern crate sdl2_ttf;
extern crate portmidi as pm;
extern crate num;
extern crate test;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use std::path::{Path, PathBuf};
use std::env;

mod looper;
mod traits;
mod midi;
mod graphics_primitives;
mod hardcode;
mod measure;
mod ui;
mod screen;
mod config;
mod error;
mod path;

use midi::PortMidiNoteTracker;
use ui::Popup;
use screen::*;
use config::Config;
use hardcode::*;
use error::Result;

fn print_devices(pm: &pm::PortMidi) {
    for dev in pm.devices().unwrap() {
        println!("{}", dev);
    }
}

fn config_path() -> Result<PathBuf> {
    env::home_dir()
        .ok_or("Home directory not found".into())
        .map(|config_dir| config_dir.join(hardcode::CONFIG_FILE_NAME))
}

fn main() {
    let context = pm::PortMidi::new().unwrap();

    let (input_id, output_id) = {
        let args: Vec<String> = std::env::args().collect();

        if args.len() < 2 {
            print_devices(&context);
            println!("Usage: ./dimooper <input-port> <output-port>");
            std::process::exit(1);
        }

        (args[1].trim().parse().unwrap(), args[2].trim().parse().unwrap())
    };

    let in_info = context.device(input_id).unwrap();
    println!("Listening on: {} {}", in_info.id(), in_info.name());
    let in_port = context.input_port(in_info, 1024).unwrap();

    let out_info = context.device(output_id).unwrap();
    println!("Sending recorded events: {} {}",
             out_info.id(),
             out_info.name());
    let out_port = context.output_port(out_info, 1024).unwrap();

    let window_width = RATIO_WIDTH * RATIO_FACTOR;
    let window_height = RATIO_HEIGHT * RATIO_FACTOR;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();
    let ttf_context = sdl2_ttf::init().unwrap();

    let window = video_subsystem.window("Dimooper", window_width, window_height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let renderer = window.renderer().build().unwrap();

    let bpm_popup = {
        let font = ttf_context.load_font(Path::new(TTF_FONT_PATH), 50).unwrap();
        Popup::new(font)
    };

    let event_pump = sdl_context.event_pump().unwrap();

    let looper = looper::Looper::new(PortMidiNoteTracker::new(out_port));

    let config = config_path()
        .and_then(|path| Config::load(path.as_path()))
        // TODO(f19dedf2-afdb-4cd9-9dab-20ebbe89fd9d): Output the path to the config file
        .map_err(|err| { println!("[WARNING] Cannot load config: {}. Using default config.", err); err })
        .unwrap_or_default();

    let mut event_loop = EventLoop::new(timer_subsystem, event_pump, in_port, renderer);
    event_loop.run(LooperScreen::<PortMidiNoteTracker>::new(looper, bpm_popup));

    config_path()
        .and_then(|path| config.save(path.as_path()))
        .expect("Cannot save the config file");
}
