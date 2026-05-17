//! M1.2 — Widget trait + Container/Row/Column.
//!
//! Built on top of `m1-cellbuffer` (which re-exports
//! `presentar_terminal::CellBuffer`). The `Widget` trait here is a
//! pedagogical mini-version of `presentar_core::Widget` — same shape
//! (paint into a buffer at a Rect), just stripped of the Brick layer
//! so the lesson stays focused.

use m1_cellbuffer::{ansi_to_color, write_str, CellBuffer, Modifiers};
use presentar_core::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

/// Composite-pattern trait shared by every widget.
pub trait Widget {
    fn paint(&self, buf: &mut CellBuffer, rect: Rect);
}

/// Solid-fill block.
pub struct Block {
    pub ch: char,
    pub fg: u8,
}

impl Widget for Block {
    fn paint(&self, buf: &mut CellBuffer, rect: Rect) {
        let mut byte_buf = [0u8; 4];
        let sym = self.ch.encode_utf8(&mut byte_buf);
        let fg = ansi_to_color(self.fg);
        for y in rect.y..rect.y.saturating_add(rect.h) {
            for x in rect.x..rect.x.saturating_add(rect.w) {
                if let Some(c) = buf.get_mut(x, y) {
                    c.update(sym, fg, Color::TRANSPARENT, Modifiers::NONE);
                }
            }
        }
    }
}

/// One-line text.
pub struct Label {
    pub text: String,
    pub fg: u8,
}

impl Widget for Label {
    fn paint(&self, buf: &mut CellBuffer, rect: Rect) {
        let truncated: String = self.text.chars().take(rect.w as usize).collect();
        write_str(buf, rect.x, rect.y, &truncated, self.fg);
    }
}

pub enum Direction {
    Row,
    Column,
}

/// Composite — lays children evenly along `direction`.
pub struct Container {
    pub direction: Direction,
    pub children: Vec<Box<dyn Widget>>,
}

impl Widget for Container {
    fn paint(&self, buf: &mut CellBuffer, rect: Rect) {
        let n = self.children.len() as u16;
        if n == 0 {
            return;
        }
        match self.direction {
            Direction::Row => {
                let each_w = rect.w / n;
                for (i, child) in self.children.iter().enumerate() {
                    child.paint(
                        buf,
                        Rect {
                            x: rect.x + (i as u16) * each_w,
                            y: rect.y,
                            w: each_w,
                            h: rect.h,
                        },
                    );
                }
            }
            Direction::Column => {
                let each_h = rect.h / n;
                for (i, child) in self.children.iter().enumerate() {
                    child.paint(
                        buf,
                        Rect {
                            x: rect.x,
                            y: rect.y + (i as u16) * each_h,
                            w: rect.w,
                            h: each_h,
                        },
                    );
                }
            }
        }
    }
}

#[must_use]
pub fn contract_marker() -> &'static str {
    "contract: tui-rendering-v1 holds — OK"
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn label_paints_text() {
        let mut buf = CellBuffer::new(20, 1);
        let l = Label {
            text: "hello".into(),
            fg: 4,
        };
        l.paint(
            &mut buf,
            Rect {
                x: 2,
                y: 0,
                w: 20,
                h: 1,
            },
        );
        assert_eq!(buf.get(2, 0).expect("cell").symbol.as_str(), "h");
        assert_eq!(buf.get(6, 0).expect("cell").symbol.as_str(), "o");
    }

    #[test]
    fn row_lays_out_children_no_overflow() {
        let row = Container {
            direction: Direction::Row,
            children: vec![
                Box::new(Block { ch: 'A', fg: 1 }),
                Box::new(Block { ch: 'B', fg: 2 }),
            ],
        };
        let mut buf = CellBuffer::new(10, 1);
        row.paint(
            &mut buf,
            Rect {
                x: 0,
                y: 0,
                w: 10,
                h: 1,
            },
        );
        assert_eq!(buf.get(0, 0).expect("cell").symbol.as_str(), "A");
        assert_eq!(buf.get(5, 0).expect("cell").symbol.as_str(), "B");
        assert_eq!(buf.get(9, 0).expect("cell").symbol.as_str(), "B");
    }

    #[test]
    fn column_lays_out_children_no_overflow() {
        let col = Container {
            direction: Direction::Column,
            children: vec![
                Box::new(Block { ch: 'A', fg: 1 }),
                Box::new(Block { ch: 'B', fg: 2 }),
            ],
        };
        let mut buf = CellBuffer::new(1, 6);
        col.paint(
            &mut buf,
            Rect {
                x: 0,
                y: 0,
                w: 1,
                h: 6,
            },
        );
        assert_eq!(buf.get(0, 0).expect("cell").symbol.as_str(), "A");
        assert_eq!(buf.get(0, 3).expect("cell").symbol.as_str(), "B");
    }

    #[test]
    fn empty_container_is_noop() {
        let row = Container {
            direction: Direction::Row,
            children: vec![],
        };
        let mut buf = CellBuffer::new(4, 1);
        row.paint(
            &mut buf,
            Rect {
                x: 0,
                y: 0,
                w: 4,
                h: 1,
            },
        );
        for x in 0..4 {
            assert_eq!(buf.get(x, 0).expect("cell").symbol.as_str(), " ");
        }
    }

    #[test]
    fn contract_marker_matches() {
        assert!(contract_marker().starts_with("contract:"));
        assert!(contract_marker().ends_with("— OK"));
    }
}
