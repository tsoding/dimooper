use std::path::Path;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;

use error::Result;
use midi::*;
use hardcode::*;
use looper::Looper;

pub struct KeyboardLayout {
    layout: HashMap<Keycode, u8>,
}

impl KeyboardLayout {
    pub fn from_slice(mappings: &[(Keycode, u8)]) -> KeyboardLayout {
        KeyboardLayout {
            layout: mappings.iter().cloned().collect()
        }
    }

    // pub fn from_path(path: &Path) -> Result<KeyboardLayout> {

    // }

    pub fn key_down<NoteTracker: MidiNoteTracker>(&self,
                                                  looper: &mut Looper<NoteTracker>,
                                                  keycode: &Keycode,
                                                  timestamp: u32) {
        self.layout
            .get(keycode)
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
        self.layout
            .get(keycode)
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
