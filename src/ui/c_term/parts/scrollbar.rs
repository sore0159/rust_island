use super::super::output::{new_rgb, Rect, Style};

#[derive(Debug)]
pub struct Scrollbar {
    pub start: (u16, u16),
    pub len: u16,
    pub vert: bool,
    pub style: Style,
    pub view_size: usize,
    pub total_size: usize,
    pub skipped_lines: usize,
}

impl Scrollbar {
    pub fn new() -> Self {
        let mut s: Style = Default::default();
        s.background_color = Some(new_rgb(40, 40, 40));
        s.foreground_color = Some(new_rgb(255, 255, 255));
        Scrollbar {
            start: (0, 0),
            len: 0,
            vert: true,
            style: s,
            view_size: 0,
            total_size: 0,
            skipped_lines: 0,
        }
    }

    pub fn draw(&self, r: &mut Rect) {
        if self.view_size >= self.total_size {
            return;
        }
        let view_end = self.skipped_lines + self.view_size;
        let chunk_size = (self.total_size as f64) / (2.0 * self.view_size as f64);
        for i in 0..self.view_size {
            let (c1, c2) = (chunk_size * (2 * i) as f64, chunk_size * (2 * i + 1) as f64);
            let (h1, h2);
            h1 = ((view_end as f64) >= c1) && ((self.skipped_lines as f64) <= c1 + chunk_size);
            h2 = ((view_end as f64) >= c2) && ((self.skipped_lines as f64) <= c2 + chunk_size);

            let coord = if self.vert {
                (self.start.0, self.start.1 + i as u16)
            } else {
                (self.start.0 + i as u16, self.start.1)
            };
            let c = match (h1, h2) {
                (false, false) => ' ', //'\u{2591}',
                (true, true) => '\u{2588}',
                (true, false) => {
                    if self.vert {
                        '\u{2580}'
                    } else {
                        '\u{258c}'
                    }
                }
                (false, true) => {
                    if self.vert {
                        '\u{2584}'
                    } else {
                        '\u{2590}'
                    }
                }
            };
            r.imprint_at(coord, c, &self.style);
        }
    }
}
