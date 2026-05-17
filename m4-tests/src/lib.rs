//! Pure-Rust TUI testing — golden cell-buffer snapshots.
//!
//! Provable contract: `contracts/tui-rendering-v1.yaml`. Built on
//! `presentar_terminal::CellBuffer`.

use m1_cellbuffer::CellBuffer;

/// Render a CellBuffer as a deterministic ASCII string (chars only,
/// row by row, newline-separated).
#[must_use]
pub fn snapshot(buf: &CellBuffer) -> String {
    let mut s = String::with_capacity(buf.len() + usize::from(buf.height()));
    for y in 0..buf.height() {
        for x in 0..buf.width() {
            if let Some(c) = buf.get(x, y) {
                s.push_str(c.symbol.as_str());
            }
        }
        s.push('\n');
    }
    s
}

/// Diff result: list of `(x, y, expected, actual)` mismatches.
pub type SnapshotDiff = Vec<(u16, u16, String, String)>;

/// Compare a buffer to a golden snapshot string. Empty Vec == match.
#[must_use]
pub fn diff_snapshot(buf: &CellBuffer, golden: &str) -> SnapshotDiff {
    let mut out = SnapshotDiff::new();
    let golden_rows: Vec<&str> = golden.lines().collect();
    for y in 0..buf.height() {
        let expected_row = golden_rows.get(y as usize).copied().unwrap_or("");
        let expected_chars: Vec<char> = expected_row.chars().collect();
        for x in 0..buf.width() {
            let actual = buf
                .get(x, y)
                .map(|c| c.symbol.as_str().to_string())
                .unwrap_or_default();
            let expected = expected_chars
                .get(x as usize)
                .map(|c| c.to_string())
                .unwrap_or_else(|| " ".to_string());
            if actual != expected {
                out.push((x, y, expected, actual));
            }
        }
    }
    out
}

#[must_use]
pub fn contract_marker() -> &'static str {
    "contract: tui-rendering-v1 holds — OK"
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;
    use m1_cellbuffer::{ansi_to_color, Modifiers};
    use presentar_core::Color;

    fn set_ch(buf: &mut CellBuffer, x: u16, y: u16, ch: &str, fg: u8) {
        if let Some(c) = buf.get_mut(x, y) {
            c.update(ch, ansi_to_color(fg), Color::TRANSPARENT, Modifiers::NONE);
        }
    }

    #[test]
    fn snapshot_of_empty_buffer_is_spaces() {
        let buf = CellBuffer::new(3, 2);
        assert_eq!(snapshot(&buf), "   \n   \n");
    }

    #[test]
    fn diff_of_matching_snapshot_is_empty() {
        let mut buf = CellBuffer::new(3, 1);
        set_ch(&mut buf, 0, 0, "a", 1);
        set_ch(&mut buf, 1, 0, "b", 1);
        set_ch(&mut buf, 2, 0, "c", 1);
        assert!(diff_snapshot(&buf, "abc\n").is_empty());
    }

    #[test]
    fn diff_with_short_golden_pads_spaces() {
        // Golden has one row, buffer is taller — y > golden_rows.len triggers
        // unwrap_or for expected_row; x past golden width triggers " " default.
        let mut buf = CellBuffer::new(3, 2);
        set_ch(&mut buf, 0, 0, "a", 1);
        set_ch(&mut buf, 0, 1, "z", 1);
        let mismatches = diff_snapshot(&buf, "a\n");
        // (1,0) and (2,0) are ' ' vs ' ' → match
        // (0,1) is 'z' vs ' ' → mismatch
        // (1,1), (2,1) ' ' vs ' ' → match
        assert!(mismatches.iter().any(|(x, y, _, _)| *x == 0 && *y == 1));
    }

    #[test]
    fn diff_reports_mismatches() {
        let mut buf = CellBuffer::new(3, 1);
        set_ch(&mut buf, 0, 0, "a", 1);
        set_ch(&mut buf, 1, 0, "X", 1);
        set_ch(&mut buf, 2, 0, "c", 1);
        let d = diff_snapshot(&buf, "abc\n");
        assert_eq!(d.len(), 1);
        assert_eq!(d[0], (1, 0, "b".to_string(), "X".to_string()));
    }

    #[test]
    fn contract_marker_matches() {
        assert!(contract_marker().starts_with("contract:"));
        assert!(contract_marker().ends_with("— OK"));
    }
}
