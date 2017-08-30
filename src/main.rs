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
mod fundamental;

use midi::PortMidiNoteTracker;
use pm::PortMidiDeviceId as DeviceId;
use ui::Popup;
use screen::*;
use config::Config;
use hardcode::*;
use error::{Result, OrExit};

fn config_path() -> Result<PathBuf> {
    env::home_dir()
        .ok_or("Home directory not found".into())
        .map(|config_dir| config_dir.join(hardcode::CONFIG_FILE_NAME))
}

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

fn create_popup(ttf_context: &sdl2_ttf::Sdl2TtfContext) -> Result<Popup> {
    let font = try!(ttf_context.load_font(Path::new(TTF_FONT_PATH), 50));
    let popup = Popup::new(font);
    Ok(popup)
}

fn main() {
    use clap::{App, AppSettings, Arg, SubCommand};

    let ttf_context = sdl2_ttf::init().or_exit("Unable to initialize SDL_ttf context");
    let context = pm::PortMidi::new().or_exit("Unable to initialize PortMidi");
    let devices = context.devices()
        .or_exit("Unable to get list of devices")
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n");

    let input_id_arg = Arg::with_name("INPUT_ID")
        .help("Input device id")
        .index(1)
        .required(true);

    let output_id_arg = Arg::with_name("OUTPUT_ID")
        .help("Output device id")
        .index(2)
        .required(true);

    let ids = &[input_id_arg, output_id_arg];

    let matches = App::new("Dimooper")
        .about("Digital music looper")
        .after_help(format!("Avaliable devices:\n{}", devices).as_ref())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("looper")
            .about("Looper mode")
            .args(ids))
        .subcommand(SubCommand::with_name("keyboard")
            .about("Keyboard configuration mode")
            .args(ids))
        .get_matches();

    let (mode, matches) = matches.subcommand();
    let (input_id, output_id) = matches.map(|matches| {
        let input_id = matches.value_of("INPUT_ID")
            .unwrap()   // arg is required
            .parse()
            .or_exit("Unable to parse input device id");
        let output_id = matches.value_of("OUTPUT_ID")
            .unwrap()
            .parse()
            .or_exit("Unable to parse output device id");

        (input_id, output_id)
    }).unwrap(); // subcommand is required

    let mut config = config_path()
        .and_then(|path| Config::load(path.as_path()))
        // TODO(f19dedf2-afdb-4cd9-9dab-20ebbe89fd9d): Output the path to the config file
        .map_err(|err| { println!("[WARNING] Cannot load config: {}. Using default config.", err); err })
        .unwrap_or_default();

    let mut event_loop = create_event_loop(&context, input_id)
        .or_exit("Initialization error");
    match mode {
        "looper" => {
            let bpm_popup = create_popup(&ttf_context).or_exit("Unable to create popup");
            let looper = create_looper(&context, output_id)
                .or_exit("Looper initialization error");
            event_loop.run(LooperScreen::<PortMidiNoteTracker>::new(looper, bpm_popup, &config))
        },
        "keyboard" => {
            config = event_loop.run(KeyboardScreen::new(config));
        },
        _ => unreachable!()
    }

    config_path()
        .and_then(|path| config.save(path.as_path()))
        .expect("Cannot save the config file");
}
