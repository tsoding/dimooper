use pm::types::MidiMessage;
use pm::types::MidiEvent;

#[derive(PartialEq)]
pub enum MessageType {
    NoteOn,
    NoteOff,
    Other,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TypedMidiMessage {
    NoteOn {channel: u8, key: u8},
    NoteOff {channel: u8, key: u8},
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TypedMidiEvent {
    message: TypedMidiMessage,
    timestamp: u32,
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
        }),

        MessageType::NoteOff => Some(TypedMidiMessage::NoteOff {
            channel: get_note_channel(raw_message),
            key: get_note_key(raw_message),
        }),

        MessageType::Other => None,
    }
}

pub fn get_message_type_code(message: &MidiMessage) -> u8 {
    message.status & 0b11110000
}

pub fn get_message_type(message: &MidiMessage) -> MessageType {
    match get_message_type_code(message) {
        0b10010000 => MessageType::NoteOn,
        0b10000000 => MessageType::NoteOff,
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
