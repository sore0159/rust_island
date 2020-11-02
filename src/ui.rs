pub mod c_term;
pub mod mockup;

use anyhow::Result;

pub struct Canvas {
    pub stdout: c_term::Stdout,
}

impl Canvas {
    pub fn new() -> Result<Self> {
        let stdout = c_term::Stdout::new()?;
        Ok(Canvas { stdout: stdout })
    }
}

pub type Event = c_term::Key;

pub struct EventStream {
    pub stdin: c_term::Stdin,
}

impl EventStream {
    pub fn new() -> Result<EventStream> {
        let stdin = c_term::Stdin::new();
        Ok(EventStream { stdin: stdin })
    }
}

impl Iterator for EventStream {
    type Item = crate::ui::Event;
    fn next(&mut self) -> Option<crate::ui::Event> {
        self.stdin.next()
    }
}
