extern crate anyhow;
extern crate crossterm;
extern crate rand;

#[cfg(test)]
mod tests;

pub mod data;
pub mod state;
pub mod ui;

use anyhow::Result;
use crossterm::ExecutableCommand;

fn main() -> Result<()> {
    println!("HELLO");
    //let mut stack = ui::mockup::mock4::new_mock4();
    //stack.run();
    let mut stdout = std::io::stdout();
    let (x, y) = crossterm::terminal::size()?;
    //stdout.execute(crossterm::terminal::SetSize(x - 1, y - 1))?;
    stdout.execute(crossterm::terminal::SetSize(x, y))?;
    use crossterm::style::{Colorize, Styler};
    println!(
        "{} {} {}",
        "Bold".bold(),
        "Red".red().on_blue(),
        "Not Bold!"
    );

    use crossterm::cursor::MoveTo;
    use crossterm::queue;
    use crossterm::style::{PrintStyledContent, StyledContent};
    let v = vec![
        StyledContent::new(Default::default(), "S1"),
        StyledContent::new(Default::default(), "S2"),
    ];
    use std::io::Write;
    queue!(stdout, MoveTo(1, 30))?;
    for s in v {
        queue!(stdout, PrintStyledContent(s))?;
    }
    stdout.flush()?;

    Ok(())
}
