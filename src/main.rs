// FIXME(#139): Deny warnings only on CI
// #![deny(warnings)]
// FIXME(#160): fix build with clippy
// #![feature(plugin)]
// #![plugin(clippy)]

extern crate sdl2;
extern crate sdl2_ttf;
extern crate portmidi as pm;
extern crate num;
extern crate rustc_serialize;

use std::path::{Path, PathBuf};
use std::env;

use sdl2::event::Event;

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

use midi::{AbsMidiEvent, PortMidiNoteTracker};
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
    let mut timer_subsystem = sdl_context.timer().unwrap();
    let ttf_context = sdl2_ttf::init().unwrap();

    let window = video_subsystem.window("Dimooper", window_width, window_height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();

    let bpm_popup = {
        let font = ttf_context.load_font(Path::new(TTF_FONT_PATH), 50).unwrap();
        Popup::new(font)
    };

    let mut event_pump = sdl_context.event_pump().unwrap();

    let looper = looper::Looper::new(PortMidiNoteTracker::new(out_port));
    let mut running = true;

    let mut previuos_ticks = timer_subsystem.ticks();

    let config = config_path()
        .and_then(|path| Config::load(path.as_path()))
        // TODO(f19dedf2-afdb-4cd9-9dab-20ebbe89fd9d): Output the path to the config file
        .map_err(|err| { println!("[WARNING] Cannot load config: {}. Using default config.", err); err })
        .unwrap_or_default();

    let mut current_screen = LooperScreen::<PortMidiNoteTracker>::new(looper, bpm_popup);

    while running {
        let current_ticks = timer_subsystem.ticks();
        let delta_time = current_ticks - previuos_ticks;
        previuos_ticks = current_ticks;

        let sdl_events: Vec<Event> = event_pump.poll_iter().collect();
        current_screen.handle_sdl_events(&sdl_events);


        if let Ok(Some(raw_midi_events)) = in_port.read_n(1024) {
            let midi_events: Vec<AbsMidiEvent> = raw_midi_events
                .iter()
                .filter_map(|e| midi::parse_midi_event(e))
                .collect();
            current_screen.handle_midi_events(&midi_events);
        }

        if let Some(_) = current_screen.update(delta_time) {
            running = false;
        }

        current_screen.render(&mut renderer);
        renderer.present();

        std::thread::sleep(std::time::Duration::from_millis(EVENT_LOOP_SLEEP_TIMEOUT));
    }

    config_path()
        .and_then(|path| config.save(path.as_path()))
        .expect("Cannot save the config file");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello() {
        assert!(true);
    }
}
