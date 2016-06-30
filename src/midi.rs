use pm::types::MidiMessage;
use pm::types::MidiEvent;

const NOTE_ON_STATUS: u8 = 0b10010000;
const NOTE_OFF_STATUS: u8 = 0b10000000;

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
                }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TypedMidiEvent {
    pub message: TypedMidiMessage,
    pub timestamp: u32,
}

#[derive(Clone, Copy)]
pub struct Note {
    pub start_timestamp: u32,
    pub end_timestamp: u32,
    pub key: u8,
    pub channel: u8,
    pub velocity: u8,
}

pub fn parse_midi_event(raw_event: &MidiEvent) -> Option<TypedMidiEvent> {
    parse_midi_message(&raw_event.message)
        .map(|message| TypedMidiEvent {
            message: message,
            timestamp: raw_event.timestamp,
        })
}

pub fn parse_midi_message(raw_message: &MidiMessage) -> Option<TypedMidiMessage> {
    match get_message_type(raw_message) {
        MessageType::NoteOn => Some(TypedMidiMessage::NoteOn {
            channel: get_note_channel(raw_message),
            key: get_note_key(raw_message),
            velocity: get_note_velocity(raw_message),
        }),

        MessageType::NoteOff => Some(TypedMidiMessage::NoteOff {
            channel: get_note_channel(raw_message),
            key: get_note_key(raw_message),
            velocity: get_note_velocity(raw_message),
        }),

        MessageType::Other => None,
    }
}

pub fn events_to_notes(replay_buffer: &[TypedMidiEvent]) -> Vec<Note> {
    let mut note_tracker: [[Option<Note>; 128]; 16] = [[None; 128]; 16];
    let mut result = Vec::new();

    for event in replay_buffer {
        match event.message {
            TypedMidiMessage::NoteOn { channel, key, velocity } => {
                match note_tracker[channel as usize][key as usize] {
                    Some(mut note) => {
                        note.end_timestamp = event.timestamp;
                        result.push(note);

                        note.start_timestamp = event.timestamp;
                        note_tracker[channel as usize][key as usize] = Some(note);
                    }
                    None => note_tracker[channel as usize][key as usize] = Some(Note {
                        start_timestamp: event.timestamp,
                        end_timestamp: 0,
                        key: key,
                        channel: channel,
                        velocity: velocity,
                    }),
                }
            },

            TypedMidiMessage::NoteOff { channel, key, .. } => {
                if let Some(mut note) = note_tracker[channel as usize][key as usize] {
                    note.end_timestamp = event.timestamp;
                    result.push(note);
                    note_tracker[channel as usize][key as usize] = None;
                }
            }
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
