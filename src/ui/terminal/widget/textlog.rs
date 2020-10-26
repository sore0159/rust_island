use crate::ui::terminal::{
    parts::{text, title, Scrollbar},
    widget, Key,
};

use widget::{data::WidgetData, Widget};

pub struct TextLog {
    pub w_data: WidgetData,
    pub size: (usize, usize),
    pub padding: (u16, u16),
    pub log: Vec<String>,
    pub last: Vec<String>,
    pub scroll_val: usize,
    pub keep_titles: usize,
    pub last_hold: usize,
    pub scrollbar: Scrollbar,
}

impl TextLog {
    pub fn from_widgetdata(wd: WidgetData) -> Self {
        let w = wd.rect.size.0 - 2;
        let h = wd.rect.size.1 - 2;
        let keep = wd.titles.len();
        let mut sb = Scrollbar::new();
        sb.start = (w, 2);
        TextLog {
            w_data: wd,
            size: (w as usize, h as usize),
            padding: (1, 4),
            log: Vec::new(),
            last: Vec::new(),
            scroll_val: 0,
            last_hold: 0,
            keep_titles: keep,
            scrollbar: sb,
        }
    }

    pub fn adjust_rect(&mut self) {
        self.draw_log();
        self.w_data.apply_border();
        self.draw_scrollbar();
    }
    pub fn draw_log(&mut self) {
        use title::Flair;
        self.w_data.borders.bars.truncate(4);
        self.w_data.titles.truncate(self.keep_titles);
        let mut y = 1 + self.size.1;
        let m = self.w_data.rect.default_style.to_mod();
        if self.last.len() > self.size.1 {
            let t = "Showing Only Last Event";
            self.w_data.add_title(
                t,
                (self.size.0 - t.chars().count()) as u16 / 2,
                1,
                false,
                Some(Flair::WedgeDown),
            );
            self.scrollbar.len = self.size.1 as u16;
            self.scrollbar.view_size = self.size.1;
            self.scrollbar.total_size = self.last.len();
            self.scrollbar.skipped_lines = self.last.len() - self.scroll_val - self.size.1;

            //self.w_data.apply_border();
            for (i, line) in self.last.iter().rev().skip(self.scroll_val).enumerate() {
                if i == self.size.1 {
                    return;
                }
                self.w_data
                    .rect
                    .apply_str((2, (y - i) as u16), &line, Some(&m));
            }
            return;
        }
        for line in self.last.iter().rev() {
            self.w_data.rect.apply_str((2, y as u16), &line, Some(&m));
            y -= 1;
            if y < 2 {
                break;
            }
        }
        if y <= 2 {
            if y == 2 {
                self.w_data.borders.add_bar(2, false, 0, 0);
            }
            let t = "Showing Only Last Event";
            self.w_data.add_title(
                t,
                (self.size.0 - t.chars().count()) as u16 / 2,
                y as u16,
                false,
                Some(Flair::WedgeDown),
            );

            //self.w_data.apply_border();
            return;
        }
        if !self.last.is_empty() {
            self.w_data.borders.add_bar(y as u16, false, 0, 0);
            let t = "Last Event";
            self.w_data.add_title(
                t,
                (self.size.0 - t.chars().count()) as u16 / 2,
                y as u16,
                false,
                Some(Flair::WedgeDown),
            );
            let t2 = "Prior Events";
            self.w_data.add_title(
                t2,
                (self.size.0 - t2.chars().count() - 3) as u16,
                1,
                false,
                Some(Flair::WedgeDown),
            );
            self.scrollbar.view_size = self.size.1 - self.last.len() - 1;
            y -= 1;
        } else {
            let t2 = "All Events";
            self.w_data.add_title(
                t2,
                //(self.size.0 - t2.chars().count() - 3) as u16,
                10,
                1,
                false,
                Some(Flair::WedgeDown),
            );
            self.scrollbar.view_size = self.size.1;
        }
        self.scrollbar.len = self.scrollbar.view_size as u16;
        self.scrollbar.total_size = self.log.len();
        if self.log.len() >= (self.scroll_val + self.scrollbar.view_size) {
            self.scrollbar.skipped_lines =
                self.log.len() - (self.scroll_val + self.scrollbar.view_size);
        }

        for line in self.log.iter().rev().skip(self.scroll_val) {
            self.w_data.rect.apply_str((2, y as u16), &line, Some(&m));
            y -= 1;
            if y < 2 {
                return;
            }
        }
        let line: String = std::iter::repeat(' ').take(self.size.0).collect();
        while y > 1 {
            self.w_data.rect.apply_str((2, y as u16), &line, Some(&m));
            y -= 1;
        }
    }
    pub fn add_event(&mut self, s: impl Into<String>) {
        if !self.last.is_empty() {
            self.last_hold = self.last.len();
            self.log.append(&mut self.last);
        }
        let padder = text::Fitter::default();
        let rpad: String = std::iter::repeat(' ')
            .take(self.padding.0 as usize)
            .collect();
        let s: String = s.into().trim().to_string();
        let max_len = self.size.0 - self.padding.0 as usize - self.padding.1 as usize;
        let mut line = String::with_capacity(max_len);
        let mut word = String::with_capacity(20);
        let (mut line_len, mut word_len) = (0, 0);
        for c in s.chars().chain(Some(' ')) {
            if c.is_ascii_whitespace() {
                if word.is_empty() {
                    continue;
                }
                if line_len + word_len + 1 < max_len {
                    if !line.is_empty() {
                        line.push(' ');
                        line_len += 1;
                    }
                    line.push_str(&word);
                    word.clear();
                    line_len += word_len;
                    word_len = 0;
                    continue;
                }
                if line.is_empty() {
                    // big word
                    self.last.push(
                        padder.fit(
                            &std::iter::repeat(' ')
                                .take(self.padding.0 as usize)
                                .chain(word.chars().take(max_len))
                                .collect::<String>(),
                            self.size.0 as usize,
                        ),
                    );
                    word.clear();
                    word_len = 0;
                    continue;
                }
                self.last
                    .push(padder.fit(&format!("{}{}", rpad, line), self.size.0 as usize));
                line.clear();
                line.push_str(&word);
                word.clear();
                line_len = word_len;
                word_len = 0;
                continue;
            }
            word.push(c);
            word_len += 1;
        }
        if !line.is_empty() {
            self.last
                .push(padder.fit(&format!("{}{}", rpad, line), self.size.0 as usize));
        }
    }
    pub fn unhold_last(&mut self) {
        let mut temp: Vec<String> = Vec::with_capacity(self.last_hold);
        for _ in 0..self.last_hold {
            temp.push(self.log.pop().unwrap());
        }
        for line in temp.into_iter().rev() {
            self.last.push(line);
        }
        self.last_hold = 0;
    }
    pub fn reset_scroll_val(&mut self) {
        if self.size.1 == (self.last.len() + 1) {
            self.scroll_val = 0;
        } else if self.size.1 == self.last.len() {
            self.scroll_val = 0;
        } else if self.size.1 < self.last.len() {
            let lost_lines = self.last.len() - self.size.1;
            self.scroll_val = lost_lines;
        } else {
            self.scroll_val = 0;
        }
    }

    pub fn change_scroll_val(&mut self, mut delta: isize) -> bool {
        let down = if delta < 0 {
            delta *= -1;
            true
        } else {
            false
        };
        let mut changed = false;
        for _ in 0..delta {
            if self.change_scroll_val_once(down) {
                changed = true;
            } else {
                return changed;
            }
        }
        changed
    }
    pub fn change_scroll_val_once(&mut self, down: bool) -> bool {
        if self.size.1 == (self.last.len() + 1) {
            return false;
        } else if self.size.1 == self.last.len() {
            return false;
        } else if self.size.1 < self.last.len() {
            if down {
                if self.scroll_val > 0 {
                    self.scroll_val -= 1;
                    return true;
                } else {
                    return false;
                }
            } else {
                let lost_lines = self.last.len() - self.size.1;
                if self.scroll_val >= lost_lines {
                    return false;
                }
                self.scroll_val += 1;
                return true;
            }
        }
        if down {
            if let Some(x) = self.scroll_val.checked_sub(1) {
                self.scroll_val = x;
                return true;
            } else {
                return false;
            }
        } else {
            let mut scroll_room = self.size.1 - (self.last.len() + 1);
            if self.last.is_empty() {
                scroll_room += 1;
            }
            if scroll_room >= self.log.len() {
                return false;
            }
            let lost_lines = self.log.len() - scroll_room;
            if lost_lines <= self.scroll_val {
                return false;
            }
            self.scroll_val += 1;
        }
        true
    }

    pub fn draw_scrollbar(&mut self) {
        if self.scrollbar.len < 4 {
            return;
        }
        match self.padding.1 {
            0 => return,
            1 | 2 => self.scrollbar.start.0 = self.size.0 as u16 + 1,
            _ => self.scrollbar.start.0 = self.size.0 as u16,
        }
        if self.padding.1 == 0 {
            return;
        }

        if let Some(cl) = &self.w_data.borders.mods.fg {
            self.scrollbar.style.fg = cl.clone();
        }
        self.scrollbar.draw(&mut self.w_data.rect);
    }
}

impl Widget for TextLog {
    fn start(&mut self) -> (&str, bool) {
        self.reset_scroll_val();
        self.adjust_rect();
        self.w_data.start()
    }
    fn gain_focus(&mut self) -> &str {
        self.w_data.set_focus(true);
        self.draw_scrollbar();
        self.w_data.gen_drawstring();
        &self.w_data.updates
    }
    fn lose_focus(&mut self) -> &str {
        self.w_data.set_focus(false);
        self.draw_scrollbar();
        self.w_data.gen_drawstring();
        &self.w_data.updates
    }
    fn parse(&mut self, key: Key) -> &str {
        match key {
            Key::Down => {
                if self.change_scroll_val(-1) {
                    self.adjust_rect();
                    self.w_data.gen_drawstring();
                }
            }
            Key::PageDown => {
                if self.change_scroll_val(-10) {
                    self.adjust_rect();
                    self.w_data.gen_drawstring();
                }
            }
            Key::PageUp => {
                if self.change_scroll_val(10) {
                    self.adjust_rect();
                    self.w_data.gen_drawstring();
                }
            }
            Key::Up => {
                if self.change_scroll_val(1) {
                    self.adjust_rect();
                    self.w_data.gen_drawstring();
                }
            }
            Key::Char('\n') => {
                if !self.last.is_empty() {
                    self.add_event("");
                } else {
                    self.unhold_last();
                }
                self.reset_scroll_val();
                self.adjust_rect();
                self.w_data.gen_drawstring();
            }
            Key::Char('a') => {
                if self.last.is_empty() {
                    self.add_event("Also, Eric loves Julie");
                    self.reset_scroll_val();
                    self.adjust_rect();
                    self.w_data.gen_drawstring();
                }
            }
            _ => {} //_ => println!("GOT:{:?}", key),
        }
        &self.w_data.updates
    }
}
