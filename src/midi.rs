use pm::types::MidiMessage;

pub fn get_message_type(message: &MidiMessage) -> u8 {
    message.status & 0b11110000
}

pub fn is_note_message(message: &MidiMessage) -> bool {
    let message_type = get_message_type(message);
    message_type == 0b10000000 || message_type == 0b10010000
}

pub fn get_note_key(message: &MidiMessage) -> u8 {
    message.data1
}

pub fn get_note_channel(message: &MidiMessage) -> u8 {
    message.status & 0b00001111
}
