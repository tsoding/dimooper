// FIXME(#139): Deny warnings only on CI
// #![deny(warnings)]
#![feature(plugin)]
#![plugin(clippy)]

extern crate sdl2;
extern crate sdl2_ttf;
extern crate portmidi as pm;
extern crate num;

use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

mod looper;
mod traits;
mod midi;
mod graphicsprimitives;
mod config;
mod measure;
mod popup;

use traits::{Updatable, Renderable};
use midi::{AbsMidiEvent, TypedMidiMessage, MidiNoteTracker};
use popup::Popup;

use config::*;

fn print_devices(pm: &pm::PortMidi) {
    for dev in pm.devices().unwrap() {
        println!("{}", dev);
    }
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

    let mut bpm_popup = {
        let font = ttf_context.load_font(Path::new(TTF_FONT_PATH), 50).unwrap();
        Popup::new(font)
    };

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut looper = looper::Looper::new(MidiNoteTracker::new(out_port));
    let mut running = true;

    let mut previuos_ticks = timer_subsystem.ticks();

    while running {
        let current_ticks = timer_subsystem.ticks();
        let delta_time = current_ticks - previuos_ticks;
        previuos_ticks = current_ticks;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    running = false;
                }

                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    looper.toggle_recording();
                }

                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    looper.reset();
                }

                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    looper.undo_last_recording();
                }

                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    looper.toggle_pause();
                }

                _ => {}
            }
        }

        if let Ok(Some(events)) = in_port.read_n(1024) {
            for event in events {
                // FIXME(#149): Extract MIDI logging into a separate entity
                println!("{:?}", event.message);

                if midi::is_note_message(&event.message) &&
                    midi::get_note_channel(&event.message) == CONTROL_CHANNEL_NUMBER {
                        if midi::get_message_type(&event.message) == midi::MessageType::NoteOn &&
                       midi::get_note_key(&event.message) == CONTROL_KEY_NUMBER {
                           looper.toggle_recording();
                    }
                } else if let Some(event) = midi::parse_midi_event(&event) {
                    match event {
                        AbsMidiEvent {
                            message: TypedMidiMessage::ControlChange {
                                number: TEMPO_CHANGE_CONTROL_NUMBER,
                                value,
                                ..
                            },
                            ..
                        } => {
                            let bpm = value as u32 + 90;
                            looper.update_tempo_bpm(bpm);
                            bpm_popup.bump(format!("{:03}", bpm).as_str());
                        },

                        _ => looper.on_midi_event(&event),
                    }
                }
            }
        }

        looper.update(delta_time);
        bpm_popup.update(delta_time);

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        looper.render(&mut renderer);
        bpm_popup.render(&mut renderer);

        renderer.present();

        std::thread::sleep(std::time::Duration::from_millis(EVENT_LOOP_SLEEP_TIMEOUT));
    }

    looper.reset();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello() {
        assert!(true);
    }
}
