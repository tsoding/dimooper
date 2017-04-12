use std::{error, result};

pub type Result<T> = result::Result<T, Box<error::Error>>;
