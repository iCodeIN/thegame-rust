use std::fmt::{Debug, Display};
use std::io::{Result, Write};
use std::fs::OpenOptions;

use cursive::Cursive;

const DEBUG: bool = true;

pub fn log<S: Into<String>>(message: S) -> Result<()> {
    if !DEBUG { return Ok(()) };
    let mut message: String = message.into().to_owned();
    message.push_str("\n\n=====================================================\
                      ===========================\n\n");
    OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")?
        .write_all(message.as_bytes())?;
    Ok(())
}

pub fn logger<T: Debug>(message: &str, arg: T) -> T {
    log(format!("{}: {:?}", message, arg )).unwrap();
    arg
}