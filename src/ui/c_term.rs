pub mod cell;
pub mod parts;
pub mod rect;
pub mod style;
pub mod widget;

use std::error::Error;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Key {
    State1,
    State2,
}

pub struct Stdout;
pub struct AltStdout;

pub fn new_stdout() -> Result<Stdout, Box<dyn Error>> {
    Ok(Stdout)
}

pub fn new_alt_stdout() -> Result<AltStdout, Box<dyn Error>> {
    Ok(AltStdout)
}

impl Write for Stdout {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        Ok(1)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub struct Stdin;

impl Stdin {
    pub fn new() -> Self {
        Stdin
    }
}

impl Iterator for Stdin {
    type Item = crate::ui::Event;
    fn next(&mut self) -> Option<crate::ui::Event> {
        None
    }
}
