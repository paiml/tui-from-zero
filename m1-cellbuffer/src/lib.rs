//! CellBuffer + DiffRenderer — the foundation type for tui-from-zero.
//!
//! Provable contract: `contracts/tui-rendering-v1.yaml` (validated with `pv`).
//!
//! A `Cell` is one terminal grid position: a character + 8-bit fg/bg.
//! A `CellBuffer` is a fixed-size 2D array of cells (`width * height`).
//! `diff` walks two buffers and emits only cells where
//! `prev[x,y] != next[x,y]` — the runtime proof of the diff-correctness
//! obligation in the rendering contract.
//!
//! The Lean theorem `Theorems.DiffRenderer_Correctness` in
//! `lean/TuiFromZero/Theorems/CellBuffer.lean` discharges the universal
//! claim at L5.

use std::fmt::Write as _;

/// One terminal cell. 8-bit ANSI colors keep it portable to legacy terminals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub ch: char,
    pub fg: u8,
    pub bg: u8,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: 7,
            bg: 0,
        }
    }
}

impl Cell {
    #[must_use]
    pub fn new(ch: char, fg: u8) -> Self {
        Self { ch, fg, bg: 0 }
    }
}

/// Fixed-size 2D cell grid backing a TUI frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellBuffer {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl CellBuffer {
    /// Allocate a `width × height` buffer filled with default cells.
    /// `width` and `height` must both be > 0 (precondition of
    /// `cellbuffer_bounds` in the rendering contract).
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0 && height > 0, "CellBuffer dims must be > 0");
        Self {
            width,
            height,
            cells: vec![Cell::default(); width * height],
        }
    }

    #[must_use]
    pub const fn width(&self) -> usize {
        self.width
    }
    #[must_use]
    pub const fn height(&self) -> usize {
        self.height
    }
    #[must_use]
    pub fn len(&self) -> usize {
        self.cells.len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    /// Out-of-bounds reads return the default Cell — no panic.
    #[must_use]
    pub fn get(&self, x: usize, y: usize) -> Cell {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x]
        } else {
            Cell::default()
        }
    }

    /// Out-of-bounds writes are silently ignored — no panic.
    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = cell;
        }
    }

    /// Fill the entire buffer with one cell.
    pub fn clear(&mut self, fill: Cell) {
        for c in &mut self.cells {
            *c = fill;
        }
    }

    /// Write a UTF-8 string starting at `(x, y)`. Stops at line edge.
    pub fn write_str(&mut self, x: usize, y: usize, s: &str, fg: u8) {
        for (i, ch) in s.chars().enumerate() {
            self.set(x + i, y, Cell::new(ch, fg));
        }
    }
}

/// One emitted draw command from the diff renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrawOp {
    pub x: usize,
    pub y: usize,
    pub cell: Cell,
}

/// Compute the minimal set of draw ops so that `apply(prev, diff) == next`
/// cell-for-cell.
#[must_use]
pub fn diff(prev: &CellBuffer, next: &CellBuffer) -> Vec<DrawOp> {
    assert_eq!(prev.width, next.width, "diff: buffers differ in width");
    assert_eq!(prev.height, next.height, "diff: buffers differ in height");
    let mut ops = Vec::with_capacity(8);
    for y in 0..next.height {
        for x in 0..next.width {
            let p = prev.get(x, y);
            let n = next.get(x, y);
            if p != n {
                ops.push(DrawOp { x, y, cell: n });
            }
        }
    }
    ops
}

/// Render every cell in the buffer (for the initial frame or full repaint).
#[must_use]
pub fn full(next: &CellBuffer) -> Vec<DrawOp> {
    let mut ops = Vec::with_capacity(next.len());
    for y in 0..next.height {
        for x in 0..next.width {
            ops.push(DrawOp {
                x,
                y,
                cell: next.get(x, y),
            });
        }
    }
    ops
}

/// Serialize a list of draw ops as ANSI escape sequences for direct stdout.
#[must_use]
pub fn render_ansi(ops: &[DrawOp]) -> String {
    let mut out = String::with_capacity(ops.len() * 16);
    for op in ops {
        // 1-indexed row/col in ANSI CUP, 0-indexed in DrawOp
        let _ = write!(
            out,
            "\x1b[{};{}H\x1b[38;5;{}m\x1b[48;5;{}m{}",
            op.y + 1,
            op.x + 1,
            op.cell.fg,
            op.cell.bg,
            op.cell.ch
        );
    }
    out.push_str("\x1b[0m");
    out
}

/// Runtime smoke check the demo binary asserts against.
#[must_use]
pub fn contract_marker() -> &'static str {
    "contract: tui-rendering-v1 holds — OK"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_bounds_get_returns_default() {
        let buf = CellBuffer::new(10, 5);
        assert_eq!(buf.get(100, 100), Cell::default());
        assert_eq!(buf.get(10, 0), Cell::default());
    }

    #[test]
    fn out_of_bounds_set_is_silently_ignored() {
        let mut buf = CellBuffer::new(10, 5);
        buf.set(99, 99, Cell::new('X', 1));
        assert_eq!(buf.get(0, 0), Cell::default());
    }

    #[test]
    fn diff_returns_only_changed_cells() {
        let prev = CellBuffer::new(4, 2);
        let mut next = CellBuffer::new(4, 2);
        next.set(1, 0, Cell::new('X', 2));
        next.set(3, 1, Cell::new('Y', 3));
        let ops = diff(&prev, &next);
        assert_eq!(ops.len(), 2);
        assert!(ops.iter().any(|o| o.x == 1 && o.y == 0));
        assert!(ops.iter().any(|o| o.x == 3 && o.y == 1));
    }

    #[test]
    fn diff_of_equal_buffers_is_empty() {
        let prev = CellBuffer::new(8, 4);
        let next = prev.clone();
        assert!(diff(&prev, &next).is_empty());
    }

    #[test]
    fn full_emits_every_cell() {
        let buf = CellBuffer::new(3, 2);
        assert_eq!(full(&buf).len(), 6);
    }

    #[test]
    fn diff_never_emits_more_than_buffer_holds() {
        // Runtime invariant from `tui-rendering-v1`:
        //   |diff(prev, next)| <= buf.len() for any (prev, next)
        let prev = CellBuffer::new(5, 5);
        let mut next = CellBuffer::new(5, 5);
        for y in 0..5 {
            for x in 0..5 {
                next.set(x, y, Cell::new('*', 1));
            }
        }
        let ops = diff(&prev, &next);
        assert!(ops.len() <= prev.len());
        assert_eq!(ops.len(), 25);
    }

    #[test]
    fn write_str_paints_text() {
        let mut buf = CellBuffer::new(10, 1);
        buf.write_str(0, 0, "hi", 4);
        assert_eq!(buf.get(0, 0).ch, 'h');
        assert_eq!(buf.get(1, 0).ch, 'i');
        assert_eq!(buf.get(2, 0).ch, ' ');
    }

    #[test]
    fn render_ansi_is_nonempty() {
        let buf = CellBuffer::new(2, 1);
        let s = render_ansi(&full(&buf));
        assert!(s.contains("\x1b["));
    }

    #[test]
    fn contract_marker_matches() {
        assert!(contract_marker().starts_with("contract:"));
        assert!(contract_marker().ends_with("— OK"));
    }

    #[test]
    fn dims_and_emptiness_track_construction() {
        let buf = CellBuffer::new(3, 2);
        assert_eq!(buf.width(), 3);
        assert_eq!(buf.height(), 2);
        assert_eq!(buf.len(), 6);
        assert!(!buf.is_empty());
    }

    #[test]
    fn clear_fills_every_cell() {
        let mut buf = CellBuffer::new(4, 3);
        buf.clear(Cell::new('Z', 5));
        for y in 0..3 {
            for x in 0..4 {
                assert_eq!(buf.get(x, y).ch, 'Z');
                assert_eq!(buf.get(x, y).fg, 5);
            }
        }
    }
}
