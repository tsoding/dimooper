use rustc_serialize::json;
use std::{io, fmt};

// TODO(faea653d-7974-46ee-ac3c-b7310998a025): use io::Error instead of that
pub enum PersistenceError {
    IoError(io::Error),
    DecodingError(json::DecoderError),
    EncodingError(json::EncoderError)
}

impl From<io::Error> for PersistenceError {
    fn from(e: io::Error) -> PersistenceError {
        PersistenceError::IoError(e)
    }
}

impl From<json::DecoderError> for PersistenceError {
    fn from(e: json::DecoderError) -> PersistenceError {
        PersistenceError::DecodingError(e)
    }
}

impl From<json::EncoderError> for PersistenceError {
    fn from(e: json::EncoderError) -> PersistenceError {
        PersistenceError::EncodingError(e)
    }
}

impl fmt::Display for PersistenceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::PersistenceError::*;

        match self {
            &IoError(ref e) => write!(f, "Looper state error: {}", e),
            &DecodingError(ref e) => write!(f, "Looper state error: {}", e),
            &EncodingError(ref e) => write!(f, "Looper state error: {}", e),
        }
    }
}
