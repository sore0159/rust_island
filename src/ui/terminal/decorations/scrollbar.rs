use crate::ui::terminal::{
    rect::Rect,
    style::{Color, Style},
};

pub struct Scrollbar {
    pub start: (u16, u16),
    pub len: u16,
    pub vert: bool,
    pub style: Style,
}

impl Scrollbar {
    pub fn new(start: (u16, u16), len: u16, vert: bool) -> Self {
        let mut s: Style = Default::default();
        s.bg = Color::Rgb(40, 40, 40);
        Scrollbar {
            start: start,
            len: len,
            vert: vert,
            style: s,
        }
    }

    pub fn draw(&self, r: &mut Rect, view_size: usize, total_size: usize, view_start: usize) {
        let view_end = view_start + view_size;
        let chunk_size = (total_size as f64) / (2.0 * view_size as f64);
        for i in 0..view_size {
            let (c1, c2) = (chunk_size * (2 * i) as f64, chunk_size * (2 * i + 1) as f64);
            let (h1, h2);
            h1 = ((view_end as f64) >= c1) && ((view_start as f64) <= c1 + chunk_size);
            h2 = ((view_end as f64) >= c2) && ((view_start as f64) <= c2 + chunk_size);

            let mut cell = r.get_mut((self.start.0, self.start.1 + i as u16)).unwrap();
            cell.val = match (h1, h2) {
                (false, false) => ' ', //'\u{2591}',
                (true, true) => '\u{2588}',
                (true, false) => '\u{2580}',
                (false, true) => '\u{2584}',
            };
            cell.style.set_to(&self.style);
        }
    }
}
