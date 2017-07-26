// FIXME(#139): Deny warnings only on CI
// #![deny(warnings)]
// FIXME(#160): fix build with clippy
// #![feature(plugin)]
// #![plugin(clippy)]

extern crate sdl2;
extern crate sdl2_ttf;
extern crate portmidi as pm;
extern crate num;
extern crate clap;

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
use pm::PortMidiDeviceId;
use ui::Popup;
use screen::*;
use config::Config;
use hardcode::*;
use error::Result;

fn config_path() -> Result<PathBuf> {
    env::home_dir()
        .ok_or("Home directory not found".into())
        .map(|config_dir| config_dir.join(hardcode::CONFIG_FILE_NAME))
}

type DeviceId = PortMidiDeviceId;
type Looper = looper::Looper<PortMidiNoteTracker>;

fn create_looper(context: &pm::PortMidi, output_id: DeviceId) -> Result<Looper> { 
    let out_info = try!(context.device(output_id));
    println!("Sending recorded events: {} {}",
             out_info.id(),
             out_info.name());
    let out_port = try!(context.output_port(out_info, 1024));
    let looper = looper::Looper::new(PortMidiNoteTracker::new(out_port));
    Ok(looper)
}

fn create_event_loop(context: &pm::PortMidi, input_id: DeviceId) -> Result<EventLoop<'static>> {
    let window_width = RATIO_WIDTH  * RATIO_FACTOR;
    let window_height = RATIO_HEIGHT * RATIO_FACTOR;

    let in_info = try!(context.device(input_id));
    println!("Listening on: {} {}", in_info.id(), in_info.name());
    let in_port = try!(context.input_port(in_info, 1024));

    let sdl_context = try!(sdl2::init());
    let video_subsystem = try!(sdl_context.video());
    let timer_subsystem = try!(sdl_context.timer());

    let window = try!(video_subsystem.window("Dimooper", window_width, window_height)
        .position_centered()
        .opengl()
        .build());

    let renderer = try!(window.renderer().build());
    let event_pump = try!(sdl_context.event_pump());
    let event_loop = EventLoop::new(timer_subsystem, event_pump, in_port, renderer);
    Ok(event_loop)
}

fn init_font() -> Result<Popup> {
    let ttf_context = try!(sdl2_ttf::init());
    let font = try!(ttf_context.load_font(Path::new(TTF_FONT_PATH), 50));
    let popup = Popup::new(font);
    Ok(popup)
}

fn main() {
    use clap::{App, SubCommand, ArgMatches};

    let context = pm::PortMidi::new().expect("Unable to initialize PortMidi");
    let devices = context.devices()
        .expect("Unable to get list of devices")
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n");

    let usage = "<INPUT_ID> 'input device id' 
                 <OUTPUT_ID> 'output device id'";

    let matches = App::new("Dimooper")
        .about("Digital music looper")
        .after_help(format!("Avaliable devices:\n {}", devices).as_ref())
        .args_from_usage(usage)
        .subcommand(SubCommand::with_name("keyboard")
            .about("Open keyboard screen")
            .args_from_usage(usage))
        .get_matches();

    let mut config = config_path()
        .and_then(|path| Config::load(path.as_path()))
        // TODO(f19dedf2-afdb-4cd9-9dab-20ebbe89fd9d): Output the path to the config file
        .map_err(|err| { println!("[WARNING] Cannot load config: {}. Using default config.", err); err })
        .unwrap_or_default();

    let get_ids = |matches: &ArgMatches| -> Result<(DeviceId, DeviceId)> {
        let input_id = try!(matches.value_of("INPUT_ID")
            .unwrap()
            .parse());
        
        let output_id = try!(matches.value_of("OUTPUT_ID")
            .unwrap()
            .parse());

        Ok((input_id, output_id))
    };

    if let Some(matches) = matches.subcommand_matches("keyboard") {
        let (input_id, _output_id) = get_ids(matches)
            .expect("Unable to parse ids");
        let mut event_loop = create_event_loop(&context, input_id)
            .expect("Initialization error");
        config = event_loop.run(KeyboardLayoutScreen::new(config)); 
    } else {
        let (input_id, output_id) = get_ids(&matches)
            .expect("Unable to parse ids");
        let looper = create_looper(&context, output_id)
            .expect("Looper initialization error");
        let mut event_loop = create_event_loop(&context, input_id)
            .expect("Event loop initialization error");
        let bpm_popup = init_font().expect("Unable to initialize fonts");
        event_loop.run(LooperScreen::<PortMidiNoteTracker>::new(looper, bpm_popup, &config))       
    }

    config_path()
        .and_then(|path| config.save(path.as_path()))
        .expect("Cannot save the config file");
}
