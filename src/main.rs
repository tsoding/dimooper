extern crate sdl2;
extern crate sdl2_sys;
extern crate portmidi as pm;

use pm::types::MidiMessage;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

mod track;
mod ark;
mod looper;
mod updatable;

use looper::State;
use updatable::Updatable;

fn midi_to_color(message: &MidiMessage) -> Color {
    Color::RGB(message.status, message.data1, message.data2)
}

struct Note {
    pitch: u32,
    duration: u32,
    start: u32
}

fn update_all(updatables: &mut Vec<&mut Updatable>,
              delta_time: u32) {
    for updatable in updatables {
        updatable.update(delta_time);
    }
}

fn main() {
    let context = pm::PortMidi::new().unwrap();

    let in_info = context.device(1).unwrap();
    println!("Listening on: {} {}", in_info.id(), in_info.name());
    let in_port = context.input_port(in_info, 1024).unwrap();

    let out_info = context.device(0).unwrap();
    println!("Sending recorded events: {} {}", out_info.id(), out_info.name());
    let mut out_port = context.output_port(out_info, 1024).unwrap();

    let window_width = 800;
    let window_height = 600;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut timer_subsystem = sdl_context.timer().unwrap();

    let window = video_subsystem.window("Midi Looper", window_width, window_height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut arkanoid = ark::Arkanoid::new(window_width, window_height);
    let mut renderer = window.renderer().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut looper = looper::Looper::new(&mut out_port);
    let mut state = State::Recording;

    let mut previuos_ticks = timer_subsystem.ticks();

    while state != State::Quit {
        let current_ticks = timer_subsystem.ticks();
        let delta_time = current_ticks - previuos_ticks;
        previuos_ticks = current_ticks;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), ..  } => {
                    state = State::Quit
                },

                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    state = State::Looping;
                    looper.looping();
                }

                _ => {}
            }
        }

        if let Ok(Some(events)) = in_port.read_n(1024) {
            for event in events {
                arkanoid.set_color(midi_to_color(&event.message));
                looper.on_midi_event(&event);
            }
        }

        {
            let mut updatables: Vec<&mut Updatable> = Vec::new();
            updatables.push(&mut looper);
            updatables.push(&mut arkanoid);
            update_all(&mut updatables, delta_time);
        }
        arkanoid.render(&mut renderer);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello() {
        assert!(true);
    }
}
