use sdl2::keyboard::Keycode;
use std::collections::HashMap;
use num::ToPrimitive;

use midi::*;
use hardcode::*;
use looper::Looper;
use config::Config;

pub struct KeyboardLayout {
    layout: HashMap<u64, u8>,
}

impl KeyboardLayout {
    pub fn from_config(config: &Config) -> KeyboardLayout {
        KeyboardLayout {
            layout: config.keyboard_layout.clone()
        }
    }

    pub fn key_down<NoteTracker: MidiNoteTracker>(&self,
                                                  looper: &mut Looper<NoteTracker>,
                                                  keycode: &Keycode,
                                                  timestamp: u32) {
        keycode
            .to_u64()
            .and_then(|keyvalue| self.layout.get(&keyvalue))
            .map(|midi_key| {
                looper.on_midi_event(&AbsMidiEvent {
                    message: TypedMidiMessage::NoteOn {
                        key: *midi_key,
                        channel: KEYBOARD_MESSAGE_CHANNEL,
                        velocity: KEYBOARD_MESSAGE_VELOCITY,
                    },
                    timestamp: timestamp
                });
            });

    }

    pub fn key_up<NoteTracker: MidiNoteTracker>(&self,
                                                looper: &mut Looper<NoteTracker>,
                                                keycode: &Keycode,
                                                timestamp: u32) {
        keycode
            .to_u64()
            .and_then(|keyvalue| self.layout.get(&keyvalue))
            .map(|midi_key| {
                looper.on_midi_event(&AbsMidiEvent {
                    message: TypedMidiMessage::NoteOff {
                        key: *midi_key,
                        channel: KEYBOARD_MESSAGE_CHANNEL,
                        velocity: KEYBOARD_MESSAGE_VELOCITY,
                    },
                    timestamp: timestamp
                });
            });
    }
}
