use rustc_serialize::json;
use std::{io, fmt};

pub enum StateError {
    IoError(io::Error),
    StateDecodingError(json::DecoderError),
    StateEncodingError(json::EncoderError)
}

impl From<io::Error> for StateError {
    fn from(e: io::Error) -> StateError {
        StateError::IoError(e)
    }
}

impl From<json::DecoderError> for StateError {
    fn from(e: json::DecoderError) -> StateError {
        StateError::StateDecodingError(e)
    }
}

impl From<json::EncoderError> for StateError {
    fn from(e: json::EncoderError) -> StateError {
        StateError::StateEncodingError(e)
    }
}

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::StateError::*;

        match self {
            &IoError(ref e) => write!(f, "Looper state error: {}", e),
            &StateDecodingError(ref e) => write!(f, "Looper state error: {}", e),
            &StateEncodingError(ref e) => write!(f, "Looper state error: {}", e),
        }
    }
}
