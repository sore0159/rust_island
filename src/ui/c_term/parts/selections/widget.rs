use super::super::super::{
    widget::{data, Widget},
    Key,
};

use super::{Choice, Chooser};

pub struct BasicWidget {
    pub w_data: data::WidgetData,
    pub selections: Chooser,
}

impl BasicWidget {
    pub fn new(origin: (u16, u16), size: (u16, u16), chooser: Chooser) -> Self {
        BasicWidget {
            w_data: data::WidgetData::new(origin, size.0, size.1),
            selections: chooser,
        }
    }
    pub fn clone_choice(&self) -> Choice {
        self.selections.choice.clone()
    }
}
impl Widget for BasicWidget {
    fn start(&mut self) -> bool {
        self.w_data.set_focus(false);
        self.selections.set_focus(false);
        for text in &self.w_data.texts {
            text.draw(&mut self.w_data.rect);
        }
        self.selections.draw(&mut self.w_data.rect);
        self.w_data.focusable
    }
    fn gain_focus(&mut self) {
        self.w_data.set_focus(true);
        self.selections.set_focus(true);
        self.selections.draw(&mut self.w_data.rect);
    }
    fn lose_focus(&mut self) {
        self.w_data.set_focus(false);
        self.selections.set_focus(false);
        self.selections.draw(&mut self.w_data.rect);
    }
    fn parse(&mut self, key: Key) -> bool {
        if self.selections.parse_key(key) {
            self.selections.draw(&mut self.w_data.rect);
            true
        } else {
            false
        }
    }
    fn queue_write(&mut self, s: &mut crate::ui::c_term::Stdout) -> anyhow::Result<()> {
        self.w_data.queue_write(s)
    }
}
