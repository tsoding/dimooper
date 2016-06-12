extern crate sdl2;
extern crate sdl2_sys;
extern crate portmidi as pm;

use pm::types::MidiEvent;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Renderer;
use sdl2::rect::{Point, Rect};

mod looper;
mod updatable;
mod midi;

use midi::MessageType;
use looper::Looper;
use updatable::Updatable;

macro_rules! colors {
    ($($hex:expr),*) => {
        &[$(
            Color::RGB((($hex & 0xFF0000) >> 16) as u8,
                       (($hex & 0xFF00) >> 8) as u8,
                       ($hex & 0xFF) as u8)
        ),*]
    }
}

const EVENT_PALETTE: &'static [Color; 5] = colors![
    0xF15A5A,
    0xF0C419,
    0x4EBA6F,
    0x2D95BF,
    0x955BA5
];

struct Note {
    start_timestamp: u32,
    end_timestamp: u32,
    key: u8,
}

fn events_to_notes(record_buffer: &[MidiEvent]) -> Vec<Note> {
    let mut note_tracker: [Option<u32>; 128] = [None; 128];
    let mut result = Vec::new();

    use midi::MessageType::*;

    for event in record_buffer {
        match (midi::get_message_type(&event.message), midi::get_note_key(&event.message)) {
            (NoteOn, key) => {
                match note_tracker[key as usize] {
                    Some(start_timestamp) => {
                        result.push(Note {
                            start_timestamp: start_timestamp,
                            end_timestamp: event.timestamp,
                            key: key
                        });
                        note_tracker[key as usize] = Some(event.timestamp);
                    },
                    None => note_tracker[key as usize] = Some(event.timestamp)
                }
            },
            (NoteOff, key) => {
                match note_tracker[key as usize] {
                    Some(start_timestamp) => {
                        result.push(Note {
                            start_timestamp: start_timestamp,
                            end_timestamp: event.timestamp,
                            key: key
                        });
                        note_tracker[key as usize] = None;
                    },
                    None => ()
                }
            },
            (Other, _) => ()
        }
    }

    result
}

fn render_event(event: &MidiEvent,
                record_buffer: &[MidiEvent],
                renderer: &mut Renderer,
                window_width: u32,
                window_height: u32) {
    let row_height = window_height as f32 / 128.0;
    let n = record_buffer.len();
    let dt = (record_buffer[n - 1].timestamp - record_buffer[0].timestamp) as f32;

    let channel = midi::get_note_channel(&event.message) as usize;
    println!("channel: {}", channel);
    let color = EVENT_PALETTE[channel % EVENT_PALETTE.len()];

    let ti = (event.timestamp - record_buffer[0].timestamp) as f32;
    let x = (ti / dt * (window_width as f32 - 10.0) + 5.0) as i32;
    let y = (row_height * (127 - midi::get_note_key(&event.message)) as f32) as i32;

    renderer.set_draw_color(color);
    renderer.fill_rect(Rect::new(x, y, 10, row_height as u32)).unwrap();
}

fn render_bar(time_cursor: u32,
              record_buffer: &[MidiEvent],
              renderer: &mut Renderer,
              window_width: u32,
              window_height: u32) {
    let n = record_buffer.len();
    let dt = (record_buffer[n - 1].timestamp - record_buffer[0].timestamp) as f32;
    let x = ((time_cursor as f32) / dt * (window_width as f32 - 10.0) + 5.0) as i32;
    renderer.set_draw_color(Color::RGB(255, 255, 255));
    renderer.draw_line(Point::from((x, 0)),
                       Point::from((x, window_height as i32))).unwrap();
}

fn render_looper(looper: &Looper,
                 renderer: &mut Renderer,
                 window_width: u32,
                 window_height: u32) {
    if looper.record_buffer.len() > 1 {
        let record_buffer = &looper.record_buffer;
        let n = record_buffer.len();
        assert!(record_buffer[0].timestamp <= record_buffer[n - 1].timestamp);

        for event in record_buffer {
            render_event(&event, &record_buffer, renderer, window_width, window_height);
        }

        render_bar(looper.time_cursor, &record_buffer, renderer, window_width, window_height);
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
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), ..  } => {
                    running = false;
                },

                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    looper.looping();
                }

                Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                    looper.recording();
                }

                Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                    looper.toggle_pause();
                }

                _ => {}
            }
        }

        if let Ok(Some(events)) = in_port.read_n(1024) {
            for event in events {
                looper.on_midi_event(&event);
            }
        }

        looper.update(delta_time);
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        render_looper(&looper, &mut renderer, window_width, window_height);
        renderer.present();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello() {
        assert!(true);
    }
}
