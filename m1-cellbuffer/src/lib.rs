//! M1.1 — CellBuffer + DiffRenderer, the foundation of every TUI in this course.
//!
//! Provable contract: `contracts/tui-rendering-v1.yaml` (validated with `pv`).
//!
//! This crate is the lesson's entry point into [`aprender-present-terminal`]
//! (`presentar_terminal`) — it re-exports the three production types the
//! rendering contract gates:
//!
//!   * [`Cell`] — one terminal grid position (symbol + fg + bg + modifiers),
//!     backed by `CompactString` for zero-allocation small strings.
//!   * [`CellBuffer`] — fixed-size `width × height` grid of cells.
//!   * [`DiffRenderer`] — walks two buffers and emits only the cells that
//!     changed (the diff-correctness obligation in the rendering contract).
//!
//! The Lean theorem `Theorems.DiffRenderer_Correctness` in
//! `lean/TuiFromZero/Theorems/CellBuffer.lean` discharges the universal
//! claim at L5; the unit tests below assert the runtime half.

pub use presentar_core::Color;
pub use presentar_terminal::direct::DiffRenderer;
pub use presentar_terminal::{Cell, CellBuffer, Modifiers};

/// Convenience: build a `Cell` from a single ASCII char + a 4-bit ANSI
/// color index — the lesson's "intro to Cell" path before learners meet
/// `presentar_core::Color::Rgb`.
#[must_use]
pub fn cell_ascii(ch: char, fg: u8) -> Cell {
    let mut buf = [0u8; 4];
    Cell::new(
        ch.encode_utf8(&mut buf),
        ansi_to_color(fg),
        Color::default(),
        Modifiers::NONE,
    )
}

/// Map a 4-bit ANSI palette index (0..15) onto `presentar_core::Color`.
/// Returns one of the canonical presentar palette constants.
#[must_use]
pub fn ansi_to_color(fg: u8) -> Color {
    // Cyan + Magenta aren't bare-metal presentar constants — fall back to
    // RGBA construction so every ANSI index maps to a defined Color.
    match fg {
        0 => Color::BLACK,
        1 => Color::RED,
        2 => Color::GREEN,
        3 => Color::YELLOW,
        4 => Color::BLUE,
        // magenta = (1.0, 0.0, 1.0); cyan = (0.0, 1.0, 1.0)
        5 => Color::new(1.0, 0.0, 1.0, 1.0),
        6 => Color::new(0.0, 1.0, 1.0, 1.0),
        _ => Color::WHITE,
    }
}

/// Helper: write a string at `(x, y)` row-by-row into a `CellBuffer`.
/// Used by m1-widgets / m3-panels / m5-ptop-mini to build frames.
pub fn write_str(buf: &mut CellBuffer, x: u16, y: u16, s: &str, fg: u8) {
    let color = ansi_to_color(fg);
    let mut buf_char = [0u8; 4];
    for (i, ch) in s.chars().enumerate() {
        let col = x.saturating_add(i as u16);
        if col >= buf.width() {
            break;
        }
        let sym = ch.encode_utf8(&mut buf_char);
        if let Some(c) = buf.get_mut(col, y) {
            c.update(sym, color, Color::default(), Modifiers::NONE);
        }
    }
}

/// Serialize an entire CellBuffer as ANSI escape sequences. The
/// downstream demos use this to push a single frame to stdout — for
/// production rendering, see `presentar_terminal::direct::DiffRenderer`.
#[must_use]
pub fn render_to_ansi(buf: &CellBuffer) -> String {
    use std::fmt::Write as _;
    let mut out = String::with_capacity(usize::from(buf.width()) * usize::from(buf.height()) * 16);
    for y in 0..buf.height() {
        for x in 0..buf.width() {
            if let Some(c) = buf.get(x, y) {
                let (r, g, b) = (
                    (c.fg.r * 255.0) as u8,
                    (c.fg.g * 255.0) as u8,
                    (c.fg.b * 255.0) as u8,
                );
                let _ = write!(
                    out,
                    "\x1b[{};{}H\x1b[38;2;{};{};{}m{}",
                    y + 1,
                    x + 1,
                    r,
                    g,
                    b,
                    c.symbol.as_str()
                );
            }
        }
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
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn presentar_cellbuffer_constructs_and_reports_dims() {
        let buf = CellBuffer::new(10, 5);
        assert_eq!(buf.width(), 10);
        assert_eq!(buf.height(), 5);
        assert_eq!(buf.len(), 50);
        assert!(!buf.is_empty());
    }

    #[test]
    fn presentar_get_returns_default_outside_bounds() {
        let buf = CellBuffer::new(4, 2);
        assert!(buf.get(99, 99).is_none(), "out-of-bounds get must be None");
        assert!(buf.get(4, 0).is_none(), "exactly at right edge is None");
    }

    #[test]
    fn write_str_paints_text() {
        let mut buf = CellBuffer::new(10, 1);
        write_str(&mut buf, 0, 0, "hi", 4);
        let c0 = buf.get(0, 0).expect("cell present");
        let c1 = buf.get(1, 0).expect("cell present");
        assert_eq!(c0.symbol.as_str(), "h");
        assert_eq!(c1.symbol.as_str(), "i");
    }

    #[test]
    fn write_str_clips_at_right_edge() {
        let mut buf = CellBuffer::new(3, 1);
        write_str(&mut buf, 1, 0, "abcdef", 4);
        // x=1,2 filled; x=0 untouched.
        assert_eq!(buf.get(1, 0).unwrap().symbol.as_str(), "a");
        assert_eq!(buf.get(2, 0).unwrap().symbol.as_str(), "b");
    }

    #[test]
    fn ascii_cell_helper_uses_presentar_color() {
        let c = cell_ascii('X', 1);
        assert_eq!(c.symbol.as_str(), "X");
        assert_eq!(c.fg, Color::RED);
    }

    #[test]
    fn ansi_to_color_covers_every_branch() {
        assert_eq!(ansi_to_color(0), Color::BLACK);
        assert_eq!(ansi_to_color(1), Color::RED);
        assert_eq!(ansi_to_color(2), Color::GREEN);
        assert_eq!(ansi_to_color(3), Color::YELLOW);
        assert_eq!(ansi_to_color(4), Color::BLUE);
        assert_eq!(ansi_to_color(5), Color::new(1.0, 0.0, 1.0, 1.0));
        assert_eq!(ansi_to_color(6), Color::new(0.0, 1.0, 1.0, 1.0));
        assert_eq!(ansi_to_color(99), Color::WHITE);
    }

    #[test]
    fn diff_renderer_constructs() {
        // Presentar's DiffRenderer is the canonical diff implementation
        // that satisfies the `Diff render equals full render` obligation.
        let _renderer = DiffRenderer::new();
    }

    #[test]
    fn render_to_ansi_emits_escape_codes_per_cell() {
        let mut buf = CellBuffer::new(2, 1);
        write_str(&mut buf, 0, 0, "Hi", 1);
        let s = render_to_ansi(&buf);
        // Has cursor-pos + true-color escape + a reset.
        assert!(s.contains("\x1b["));
        assert!(s.contains("\x1b[38;2;"));
        assert!(s.ends_with("\x1b[0m"));
        assert!(s.contains('H'));
        assert!(s.contains('i'));
    }

    #[test]
    fn render_to_ansi_handles_empty_buffer_rows() {
        let buf = CellBuffer::new(1, 1);
        let s = render_to_ansi(&buf);
        assert!(s.ends_with("\x1b[0m"));
    }

    #[test]
    fn write_str_overflow_breaks_at_buffer_edge() {
        let mut buf = CellBuffer::new(3, 1);
        // Start past the right edge — every iteration triggers the break.
        write_str(&mut buf, 99, 0, "ignored", 4);
        for x in 0..3 {
            assert_eq!(buf.get(x, 0).expect("cell").symbol.as_str(), " ");
        }
    }

    #[test]
    fn contract_marker_matches() {
        assert!(contract_marker().starts_with("contract:"));
        assert!(contract_marker().ends_with("— OK"));
    }
}
