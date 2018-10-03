use std::fmt::{Debug, Display};
use std::io::{Result, Write};
use std::fs::OpenOptions;

use cursive::Cursive;

pub fn strict_log<S: Into<String>>(message: S) -> Result<()> {
    if !::DEBUG {
        return Ok(());
    };
    let mut message: String = message.into();
    message.push_str(
        "\n\n=====================================================\
         ===========================\n\n",
    );
    OpenOptions::new()
        .append(true)
        .create(true)
        .open("log.txt")?
        .write_all(message.as_bytes())?;
    Ok(())
}

pub fn log<S: Into<String>>(message: S) {
    strict_log(message).unwrap_or(());
}

pub fn logger<T: Debug>(message: &str, arg: T) -> T {
    strict_log(format!("{}: {:?}", message, arg)).unwrap();
    arg
}
