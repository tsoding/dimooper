use rustc_serialize::json;
use std::io;

pub enum StateError {
    IoError,
    ParsingError
}

impl From<io::Error> for StateError {
    fn from(_: io::Error) -> StateError {
        StateError::IoError
    }
}

impl From<json::DecoderError> for StateError {
    fn from(_: json::DecoderError) -> StateError {
        StateError::ParsingError
    }
}

impl From<json::EncoderError> for StateError {
    fn from(_: json::EncoderError) -> StateError {
        StateError::ParsingError
    }
}
