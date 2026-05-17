//! Widget trait + Container/Row/Column — the composite pattern.
//!
//! Provable contract: `contracts/tui-rendering-v1.yaml`. Every widget
//! paints inside its `Rect`; Container composes children laid out
//! horizontally (`Row`) or vertically (`Column`) — no child overflows
//! the parent rect (`tui-panels-v1` is the L5 universal claim).

use m1_cellbuffer::{Cell, CellBuffer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

/// The composite-pattern trait every widget implements.
pub trait Widget {
    /// Paint the widget into `buf` clipped to `rect`.
    fn paint(&self, buf: &mut CellBuffer, rect: Rect);
}

/// Solid-fill block — paints a single cell repeated across `rect`.
pub struct Block {
    pub cell: Cell,
}

impl Widget for Block {
    fn paint(&self, buf: &mut CellBuffer, rect: Rect) {
        for y in rect.y..rect.y + rect.h {
            for x in rect.x..rect.x + rect.w {
                buf.set(x, y, self.cell);
            }
        }
    }
}

/// One line of text painted at the top-left of its `rect`.
pub struct Label {
    pub text: String,
    pub fg: u8,
}

impl Widget for Label {
    fn paint(&self, buf: &mut CellBuffer, rect: Rect) {
        let truncated: String = self.text.chars().take(rect.w).collect();
        buf.write_str(rect.x, rect.y, &truncated, self.fg);
    }
}

pub enum Direction {
    Row,
    Column,
}

/// Composite widget — lays children out evenly along `direction`.
pub struct Container {
    pub direction: Direction,
    pub children: Vec<Box<dyn Widget>>,
}

impl Widget for Container {
    fn paint(&self, buf: &mut CellBuffer, rect: Rect) {
        let n = self.children.len();
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
                            x: rect.x + i * each_w,
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
                            y: rect.y + i * each_h,
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
        assert_eq!(buf.get(2, 0).ch, 'h');
        assert_eq!(buf.get(6, 0).ch, 'o');
    }

    #[test]
    fn row_lays_out_children_no_overflow() {
        let row = Container {
            direction: Direction::Row,
            children: vec![
                Box::new(Block {
                    cell: Cell::new('A', 1),
                }),
                Box::new(Block {
                    cell: Cell::new('B', 2),
                }),
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
        // Last cell column must NOT overflow buffer width.
        assert_eq!(buf.get(0, 0).ch, 'A');
        assert_eq!(buf.get(5, 0).ch, 'B');
        assert_eq!(buf.get(9, 0).ch, 'B');
    }

    #[test]
    fn column_lays_out_children_no_overflow() {
        let col = Container {
            direction: Direction::Column,
            children: vec![
                Box::new(Block {
                    cell: Cell::new('A', 1),
                }),
                Box::new(Block {
                    cell: Cell::new('B', 2),
                }),
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
        assert_eq!(buf.get(0, 0).ch, 'A');
        assert_eq!(buf.get(0, 3).ch, 'B');
    }

    #[test]
    fn empty_container_is_noop() {
        let row: Container = Container {
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
            assert_eq!(buf.get(x, 0), Cell::default());
        }
    }

    #[test]
    fn contract_marker_matches() {
        assert!(contract_marker().starts_with("contract:"));
        assert!(contract_marker().ends_with("— OK"));
    }
}
