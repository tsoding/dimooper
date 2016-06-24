use pm::types::MidiMessage;

#[derive(PartialEq)]
pub enum MessageType {
    NoteOn,
    NoteOff,
    Other,
}

pub struct Note {
    pub start_timestamp: u32,
    pub end_timestamp: u32,
    pub key: u8,
    pub channel: u8,
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
