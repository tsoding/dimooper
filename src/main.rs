extern crate sdl2;
extern crate sdl2_sys;
extern crate portmidi as pm;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

mod looper;
mod updatable;
mod renderable;
mod midi;
mod graphicsprimitives;

use updatable::Updatable;
use renderable::Renderable;

const EVENT_LOOP_SLEEP_TIMEOUT: u64 = 3;
const CONTROL_CHANNEL_NUMBER: u8 = 9;
const CONTROL_KEY_NUMBER: u8 = 51;

const RATIO_WIDTH: u32 = 16;
const RATIO_HEIGHT: u32 = 9;
const RATIO_FACTOR: u32 = 90;

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
            println!("Usage: ./midi-looper <input-port> <output-port>");
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
    let mut out_port = context.output_port(out_info, 1024).unwrap();

    let window_width = RATIO_WIDTH * RATIO_FACTOR;
    let window_height = RATIO_HEIGHT * RATIO_FACTOR;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut timer_subsystem = sdl_context.timer().unwrap();

    let window = video_subsystem.window("Midi Looper", window_width, window_height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut looper = looper::Looper::new(&mut out_port);
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
                    looper.toggle_pause();
                }

                _ => {}
            }
        }

        if let Ok(Some(events)) = in_port.read_n(1024) {
            for event in events {
                println!("{:?}", event.message);

                if midi::is_note_message(&event.message) &&
                    midi::get_note_channel(&event.message) == CONTROL_CHANNEL_NUMBER {
                        if midi::get_message_type(&event.message) == midi::MessageType::NoteOn &&
                       midi::get_note_key(&event.message) == CONTROL_KEY_NUMBER {
                        looper.toggle_recording();
                    }
                } else {
                    if let Some(event) = midi::parse_midi_event(&event) {
                        looper.on_midi_event(&event);
                    }
                }
            }
        }

        looper.update(delta_time);
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        looper.render(&mut renderer);
        renderer.present();

        std::thread::sleep(std::time::Duration::from_millis(EVENT_LOOP_SLEEP_TIMEOUT));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello() {
        assert!(true);
    }
}
