use termion::event::Key;
use termion::terminal_size;

use super::{border, rect, style, text};

use super::border::Borders;
use super::rect::Rect;
pub struct Mockup1 {
    pub b1: (Rect, Borders),
    pub b2: (Rect, Borders),
    pub b3: (Rect, Borders),
    selected: u8,
    pub update_str: String,
}

impl Mockup1 {
    pub fn new() -> Self {
        let (w, h) = terminal_size().unwrap();
        let mut rect1 = rect::Rect::new((1, 1), 10, h - 20);
        let mut bdr1 = border::Borders::new();
        let mut rect2 = rect::Rect::new((11, 1), w - 10, h - 20);
        let mut bdr2 = border::Borders::new();
        let mut rect3 = rect::Rect::new((1, h - 19), w, 20);
        let mut bdr3 = border::Borders::new();
        bdr1.v_bars.push(5);
        bdr1.mods.fg = Some(style::Color::Rgb(200, 100, 200));
        bdr1.mods.color_swap();
        bdr2.h_bars.push(15);
        bdr2.mods.fg = Some(style::Color::Blue);

        bdr3.v_bars.push(15);
        bdr3.h_bars.push(3);
        bdr3.mods.fg = Some(style::Color::Green);

        let mut m = style::StyleMod::new();
        m.fg = Some(style::Color::Black);
        m.bg = Some(style::Color::White);
        rect1.apply_str((2, 2), "Box One", None);
        let mut t2 = text::Text::new("    Box Two    ");
        t2.mod_style(&m);
        t2.start = (2, 2);

        let mut f3 = text::Fitter::default().middle();
        f3.val = '-';
        let mut t3 = text::Text::new("12Box6Three23");
        t3.fit(&f3, 20);
        //println!("{:?}", t3);
        t3.mod_style(&m);
        t3.start = (w / 2 - 5, 4);
        rect2.apply_text(&t2);
        rect3.apply_text(&t3);
        bdr1.draw(&mut rect1);
        bdr2.draw(&mut rect2);
        bdr3.draw(&mut rect3);
        let s = format!("{}{}{}", rect1, rect2, rect3);
        Mockup1 {
            b1: (rect1, bdr1),
            b2: (rect2, bdr2),
            b3: (rect3, bdr3),
            selected: 0,
            update_str: s,
        }
    }
}

use std::fmt::Write;
impl super::SyncTermUI for Mockup1 {
    fn to_draw(&mut self) -> &str {
        &self.update_str
    }
    fn parse(&mut self, key: Key) -> bool {
        self.update_str.clear();
        match key {
            Key::Char('q') => true,
            Key::Char('n') => {
                self.selected = (self.selected + 1) % 3;
                match self.selected {
                    0 => {
                        self.b1.1.mods.color_swap();
                        self.b3.1.mods.color_swap();
                        self.b1.1.draw(&mut self.b1.0);
                        self.b3.1.draw(&mut self.b3.0);
                        write!(self.update_str, "{}{}", self.b1.0, self.b3.0).unwrap();
                    }
                    1 => {
                        self.b1.1.mods.color_swap();
                        self.b2.1.mods.color_swap();
                        self.b1.1.draw(&mut self.b1.0);
                        self.b2.1.draw(&mut self.b2.0);
                        write!(self.update_str, "{}{}", self.b1.0, self.b2.0).unwrap();
                    }
                    _ => {
                        self.b2.1.mods.color_swap();
                        self.b3.1.mods.color_swap();
                        self.b2.1.draw(&mut self.b2.0);
                        self.b3.1.draw(&mut self.b3.0);
                        write!(self.update_str, "{}{}", self.b2.0, self.b3.0).unwrap();
                    }
                }
                false
            }
            _ => false,
        }
    }
}
