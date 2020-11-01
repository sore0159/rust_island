pub mod cell;
pub mod output;
pub mod parts;
pub mod rect;
pub mod style;
pub mod widget;

//use anyhow::Result;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Key(pub crossterm::event::KeyEvent);
pub type KeyCode = crossterm::event::KeyCode;

impl Key {
    pub fn is_char(&self, c: char) -> bool {
        self.0.code == KeyCode::Char(c)
    }
    pub fn code(&self) -> KeyCode {
        self.0.code
    }
    pub fn from_char(c: char) -> Self {
        Key(crossterm::event::KeyEvent::new(
            KeyCode::Char(c),
            crossterm::event::KeyModifiers::empty(),
        ))
    }
    pub fn from_code(c: KeyCode) -> Self {
        Key(crossterm::event::KeyEvent::new(
            c,
            crossterm::event::KeyModifiers::empty(),
        ))
    }
}

pub struct Stdout {
    pub stdout: std::io::Stdout,
    pub alt: bool,
}

impl Stdout {
    pub fn new() -> Self {
        let stdout = std::io::stdout();
        Stdout { stdout, alt: false }
    }
}

impl Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.stdout.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.stdout.flush()
    }
}

pub struct Stdin;

impl Stdin {
    pub fn new() -> Self {
        Stdin
    }
}

use crossterm::event::{self, poll, read};
use std::time::Duration;
impl Iterator for Stdin {
    type Item = crate::ui::Event;
    fn next(&mut self) -> Option<crate::ui::Event> {
        loop {
            match poll(Duration::from_secs(0)) {
                Ok(true) => {
                    match read() {
                        Err(er) => panic!("crossterm read error {}", er),
                        Ok(event::Event::Key(k)) => {
                            return Some(Key(k));
                            //
                        }
                        _ => {}
                    }
                }
                Err(er) => panic!("Crossterm poll error {}", er),
                _ => return None,
            }
        }
    }
}
