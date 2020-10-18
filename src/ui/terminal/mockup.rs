use std::io::{stdin, stdout, Write};

use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{clear, terminal_size};

use super::{border, cell, style};

pub fn draw_boxes() -> Result<(), std::io::Error> {
    let (w, h) = terminal_size()?;
    let mut rect1 = cell::Rect::new((1, 1), 10, h - 20);
    let mut bdr1 = border::Borders::new();
    let mut rect2 = cell::Rect::new((11, 1), w - 10, h - 20);
    let mut bdr2 = border::Borders::new();
    let mut rect3 = cell::Rect::new((1, h - 19), w, 20);
    let mut bdr3 = border::Borders::new();
    bdr1.v_bars.push(5);
    bdr1.mods.fg = Some(style::Color::Red);
    bdr1.mods.color_swap();
    bdr2.h_bars.push(15);
    bdr2.mods.fg = Some(style::Color::Blue);

    bdr3.v_bars.push(15);
    bdr3.h_bars.push(3);
    bdr3.mods.fg = Some(style::Color::Green);

    let mut m = style::StyleMod::new();
    m.fg = Some(style::Color::Black);
    m.bg = Some(style::Color::White);
    rect1.set_text((2, 2), "Box One");
    rect2.set_cells((2, 2), "    Box Two    ", Some(&m));
    rect3.set_cells((w / 2 - 5, 4), "Box Three", Some(&m));

    {
        let mut stdout = AlternateScreen::from(stdout().into_raw_mode()?);

        write!(stdout, "{}", clear::All)?;
        if h < 2 || w < 2 {
            stdout.flush()?;
        } else {
            bdr1.draw(&mut rect1);
            bdr2.draw(&mut rect2);
            bdr3.draw(&mut rect3);
            write!(stdout, "{}", rect1)?;
            write!(stdout, "{}", rect2)?;
            write!(stdout, "{}", rect3)?;
            stdout.flush()?;
            let mut i = 0;
            'outer: loop {
                use termion::event::Key;
                let stdin = stdin();
                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Char('q') => break 'outer,
                        Key::Char('n') => {
                            i = (i + 1) % 3;
                            match i {
                                0 => {
                                    bdr1.mods.color_swap();
                                    bdr3.mods.color_swap();
                                    bdr1.draw(&mut rect1);
                                    bdr3.draw(&mut rect3);
                                }
                                1 => {
                                    bdr1.mods.color_swap();
                                    bdr2.mods.color_swap();
                                    bdr1.draw(&mut rect1);
                                    bdr2.draw(&mut rect2);
                                }
                                _ => {
                                    bdr2.mods.color_swap();
                                    bdr3.mods.color_swap();
                                    bdr2.draw(&mut rect2);
                                    bdr3.draw(&mut rect3);
                                }
                            }
                        }
                        _ => {}
                    }
                    write!(stdout, "{}", rect1)?;
                    write!(stdout, "{}", rect2)?;
                    write!(stdout, "{}", rect3)?;

                    stdout.flush()?;
                }
            }
        }
    }
    println!("");
    Ok(())
}
