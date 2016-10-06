use std::cmp;

use pm::types::MidiMessage;
use pm::types::MidiEvent;

use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use looper::sample::QuantMidiEvent;
use measure::Quant;

mod portmidinotetracker;
mod midisink;

pub use self::portmidinotetracker::PortMidiNoteTracker;
pub use self::midisink::MidiSink;

const NOTE_ON_STATUS: u8 = 0b10010000;
const NOTE_OFF_STATUS: u8 = 0b10000000;
const CONTROL_CHANGE_STATUS: u8 = 0b10110000;

#[derive(PartialEq)]
pub enum MessageType {
    NoteOn,
    NoteOff,
    Other,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TypedMidiMessage {
    NoteOn {channel: u8, key: u8, velocity: u8},
    NoteOff {channel: u8, key: u8, velocity: u8},
    ControlChange {channel: u8, number: u8, value: u8},
}

impl Into<MidiMessage> for TypedMidiMessage {
    fn into(self) -> MidiMessage {
        match self {
            TypedMidiMessage::NoteOn {channel, key, velocity } =>
                MidiMessage {
                    status: NOTE_ON_STATUS | channel,
                    data1: key,
                    data2: velocity,
                },

            TypedMidiMessage::NoteOff {channel, key, velocity } =>
                MidiMessage {
                    status: NOTE_OFF_STATUS | channel,
                    data1: key,
                    data2: velocity,
                },

            TypedMidiMessage::ControlChange {channel, number, value} =>
                MidiMessage {
                    status: CONTROL_CHANGE_STATUS | channel,
                    data1: number,
                    data2: value,
                }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AbsMidiEvent {
    pub message: TypedMidiMessage,
    pub timestamp: u32,
}

#[derive(Clone, Copy)]
pub struct Note {
    pub start_quant: Quant,
    pub end_quant: Quant,
    pub key: u8,
    pub channel: u8,
    pub velocity: u8,
}

macro_rules! colors {
    ($($hex:expr),*) => {
        &[$(
            Color::RGB((($hex & 0xFF0000) >> 16) as u8,
                       (($hex & 0xFF00) >> 8) as u8,
                       ($hex & 0xFF) as u8)
        ),*]
    }
}

const CHANNEL_PALETTE: &'static [Color; 5] = colors![0xF15A5A, 0xF0C419, 0x4EBA6F, 0x2D95BF,
                                                     0x955BA5];

fn multiply_color_vector(color: Color, factor: f32) -> Color {
    match color {
        Color::RGB(r, g, b) | Color::RGBA(r, g, b, _) => {
            Color::RGB((r as f32 * factor) as u8,
                       (g as f32 * factor) as u8,
                       (b as f32 * factor) as u8)
        }
    }
}

impl Note {
    pub fn render(&self, renderer: &mut Renderer, Quant(window_size): Quant, window_position: Quant) {
        let window_width = renderer.viewport().width();
        let window_height = renderer.viewport().height();
        let row_height = window_height as f32 / 128.0;

        let brightness_factor =  self.velocity as f32 / 127.0;
        let base_color = CHANNEL_PALETTE[self.channel as usize % CHANNEL_PALETTE.len()];
        let color = multiply_color_vector(base_color, brightness_factor);

        let Quant(start) = self.start_quant - cmp::min(window_position, self.start_quant);
        let Quant(end) = self.end_quant - cmp::min(window_position, self.end_quant);
        let x1 = (start as f32 / window_size as f32 * (window_width as f32 - 10.0) + 5.0) as i32;
        let x2 = (end as f32 / window_size as f32 * (window_width as f32 - 10.0) + 5.0) as i32;
        let y = (row_height * (127 - self.key) as f32) as i32;

        let note_rect = Rect::new(x1, y, (x2 - x1 + 1) as u32, row_height as u32);

        renderer.set_draw_color(color);
        renderer.fill_rect(note_rect).unwrap();
    }
}

pub fn parse_midi_event(raw_event: &MidiEvent) -> Option<AbsMidiEvent> {
    parse_midi_message(&raw_event.message)
        .map(|message| AbsMidiEvent {
            message: message,
            timestamp: raw_event.timestamp,
        })
}

pub fn parse_midi_message(raw_message: &MidiMessage) -> Option<TypedMidiMessage> {
    match get_message_type_code(raw_message) {
        NOTE_ON_STATUS => Some(TypedMidiMessage::NoteOn {
            channel: get_note_channel(raw_message),
            key: get_note_key(raw_message),
            velocity: get_note_velocity(raw_message),
        }),

        NOTE_OFF_STATUS => Some(TypedMidiMessage::NoteOff {
            channel: get_note_channel(raw_message),
            key: get_note_key(raw_message),
            velocity: get_note_velocity(raw_message),
        }),

        CONTROL_CHANGE_STATUS => Some(TypedMidiMessage::ControlChange {
            channel: get_note_channel(raw_message),
            number: raw_message.data1,
            value: raw_message.data2,
        }),

        _ => None,
    }
}

pub fn events_to_notes(replay_buffer: &[QuantMidiEvent]) -> Vec<Note> {
    let mut note_tracker: [[Option<Note>; 128]; 16] = [[None; 128]; 16];
    let mut result = Vec::new();


    for event in replay_buffer {
        match event.message {
            TypedMidiMessage::NoteOn { channel, key, velocity } => {
                match note_tracker[channel as usize][key as usize] {
                    Some(mut note) => {
                        note.end_quant = event.quant;
                        result.push(note);

                        note.start_quant = event.quant;
                        note_tracker[channel as usize][key as usize] = Some(note);
                    }
                    None => note_tracker[channel as usize][key as usize] = Some(Note {
                        start_quant: event.quant,
                        end_quant: Quant(0),
                        key: key,
                        channel: channel,
                        velocity: velocity,
                    }),
                }
            },

            TypedMidiMessage::NoteOff { channel, key, .. } => {
                if let Some(mut note) = note_tracker[channel as usize][key as usize] {
                    note.end_quant = event.quant;
                    result.push(note);
                    note_tracker[channel as usize][key as usize] = None;
                }
            },

            _ => ()
        }
    }

    result
}

pub fn get_message_type_code(message: &MidiMessage) -> u8 {
    message.status & 0b11110000
}

pub fn get_message_type(message: &MidiMessage) -> MessageType {
    match get_message_type_code(message) {
        NOTE_ON_STATUS => MessageType::NoteOn,
        NOTE_OFF_STATUS => MessageType::NoteOff,
        _ => MessageType::Other
    }
}

pub fn is_note_on(message: &MidiMessage) -> bool {
    let message_type = get_message_type_code(message);
    message_type == 0b10010000
}

pub fn is_note_off(message: &MidiMessage) -> bool {
    let message_type = get_message_type_code(message);
    message_type == 0b10000000
}

pub fn is_note_message(message: &MidiMessage) -> bool {
    is_note_on(message) || is_note_off(message)
}

pub fn get_note_key(message: &MidiMessage) -> u8 {
    message.data1
}

pub fn get_note_channel(message: &MidiMessage) -> u8 {
    message.status & 0b00001111
}

pub fn get_note_velocity(message: &MidiMessage) -> u8 {
    message.data2
}
