use std::{error, result};
use std::fmt::Display;

pub type Result<T> = result::Result<T, Box<error::Error>>;

pub trait OrExit {
    type T;

    fn or_exit(self, message: &str) -> Self::T;
} 

impl<T, E> OrExit for result::Result<T, E>
    where E: Display {
    type T = T;

    fn or_exit(self, message: &str) -> Self::T {
        use std;
        use std::io::Write;

        match self {
            Ok(value) => value,
            Err(e) => {
                writeln!(&mut std::io::stderr(), "{}: {}", message, e).unwrap();
                std::process::exit(-1);
            }
        }
    }
}
