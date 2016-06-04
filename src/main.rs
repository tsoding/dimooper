extern crate sdl2;
extern crate portmidi as pm;

use std::time::Duration;
use std::thread;
use std::time::Instant;

use pm::types::{MidiEvent, MidiMessage};
use pm::OutputPort;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Renderer;

fn replay_buffer_forever(record_buffer: &[MidiEvent], out_port: &mut OutputPort) {
    loop {
        let mut some_previous_event: Option<&MidiEvent> = None;
        for event in record_buffer {
            if let Some(previous_event) = some_previous_event {
                thread::sleep(Duration::from_millis((event.timestamp - previous_event.timestamp) as u64));
            }

            out_port.write_message(event.message).unwrap();
            some_previous_event = Some(&event)
        }
    }
}

fn main2() {
    let context = pm::PortMidi::new().unwrap();
    let timeout = Duration::from_millis(10);
    let mut record_buffer = Vec::new();

    let in_info = context.device(1).unwrap();
    println!("Listening on: {} {}", in_info.id(), in_info.name());

    let in_port = context.input_port(in_info, 1024).unwrap();
    let mut recording = true;

    while let Ok(_) = in_port.poll() {
        if !recording {
            break;
        }

        if let Ok(Some(current_events)) = in_port.read_n(1024) {
            for event in current_events {
                let channel = event.message.status & 15;
                println!("Channel: {}", channel);
                println!("{:?}", event);
                if channel == 9 {
                    recording = false;
                } else {
                    record_buffer.push(event);
                }
            }
        }

        thread::sleep(timeout);
    }

    let out_info = context.device(0).unwrap();
    println!("Sending recorded events: {} {}", out_info.id(), out_info.name());

    let mut out_port = context.output_port(out_info, 1024).unwrap();

    replay_buffer_forever(&record_buffer, &mut out_port);
}

struct GameObject {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    width: u32,
    height: u32,
    color: Color,
}

impl Default for GameObject {
    fn default() -> GameObject {
        GameObject {
            x: f32::default(),
            y: f32::default(),
            vx: 1.0,
            vy: 1.0,
            width: 100,
            height: 100,
            color: Color::RGB(255, 0, 0),
        }
    }
}

impl GameObject {
    fn update(&mut self, window_width: u32, window_height: u32) {
        let velocity = 0.1;

        if self.x < 0.0 || self.x > (window_width - self.width) as f32 {
            self.vx = -self.vx;
        }

        if self.y < 0.0 || self.y > (window_height - self.height) as f32 {
            self.vy = -self.vy;
        }

        self.x += self.vx * velocity;
        self.y += self.vy * velocity;
    }

    fn render(&self, renderer: &mut Renderer) {
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        renderer.set_draw_color(self.color);
        renderer.fill_rect(Rect::new(self.x as i32, self.y as i32, self.width, self.height)).unwrap();
        renderer.present();
    }
}

fn midi_to_color(message: &MidiMessage) -> Color {
    Color::RGB(message.status, message.data1, message.data2)
}

enum State {
    Recording,
    Looping,
}

fn main() {
    let context = pm::PortMidi::new().unwrap();
    let in_info = context.device(1).unwrap();
    println!("Listening on: {} {}", in_info.id(), in_info.name());

    let in_port = context.input_port(in_info, 1024).unwrap();


    let window_width = 800;
    let window_height = 600;
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Midi Looper", window_width, window_height)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut game_object = GameObject::default();
    let mut renderer = window.renderer().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut state = State::Recording;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), ..  } => {
                    break 'running
                },
                _ => {}
            }
        }

        if let Ok(Some(events)) = in_port.read_n(1024) {
            for event in events {
                game_object.color = midi_to_color(&event.message);
            }
        }

        game_object.update(window_width, window_height);
        game_object.render(&mut renderer);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_hello() {
        assert!(true);
    }
}
