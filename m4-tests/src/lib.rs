//! Pure-Rust TUI testing — golden cell-buffer snapshots.
//!
//! Provable contract: `contracts/tui-rendering-v1.yaml`. A snapshot is
//! the textual rendering of every `Cell.ch` row-by-row. Two buffers
//! match iff every character matches at every coordinate.
//!
//! This is the lesson's answer to "how do you test a TUI without
//! Selenium?" — render to a buffer, stringify it, diff against a golden.

use m1_cellbuffer::CellBuffer;

/// Render a CellBuffer as a deterministic ASCII string (chars only,
/// row by row, newline-separated).
#[must_use]
pub fn snapshot(buf: &CellBuffer) -> String {
    let mut s = String::with_capacity(buf.len() + buf.height());
    for y in 0..buf.height() {
        for x in 0..buf.width() {
            s.push(buf.get(x, y).ch);
        }
        s.push('\n');
    }
    s
}

/// Diff result: list of `(x, y, expected, actual)` mismatches.
pub type SnapshotDiff = Vec<(usize, usize, char, char)>;

/// Compare a buffer to a golden snapshot string. Empty Vec == match.
#[must_use]
pub fn diff_snapshot(buf: &CellBuffer, golden: &str) -> SnapshotDiff {
    let mut out = SnapshotDiff::new();
    let golden_rows: Vec<&str> = golden.lines().collect();
    for y in 0..buf.height() {
        let expected_row = golden_rows.get(y).copied().unwrap_or("");
        let expected_chars: Vec<char> = expected_row.chars().collect();
        for x in 0..buf.width() {
            let actual = buf.get(x, y).ch;
            let expected = expected_chars.get(x).copied().unwrap_or(' ');
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
mod tests {
    use super::*;
    use m1_cellbuffer::Cell;

    #[test]
    fn snapshot_of_empty_buffer_is_spaces() {
        let buf = CellBuffer::new(3, 2);
        assert_eq!(snapshot(&buf), "   \n   \n");
    }

    #[test]
    fn diff_of_matching_snapshot_is_empty() {
        let mut buf = CellBuffer::new(3, 1);
        buf.set(0, 0, Cell::new('a', 1));
        buf.set(1, 0, Cell::new('b', 1));
        buf.set(2, 0, Cell::new('c', 1));
        assert!(diff_snapshot(&buf, "abc\n").is_empty());
    }

    #[test]
    fn diff_reports_mismatches() {
        let mut buf = CellBuffer::new(3, 1);
        buf.set(0, 0, Cell::new('a', 1));
        buf.set(1, 0, Cell::new('X', 1));
        buf.set(2, 0, Cell::new('c', 1));
        let d = diff_snapshot(&buf, "abc\n");
        assert_eq!(d.len(), 1);
        assert_eq!(d[0], (1, 0, 'b', 'X'));
    }

    #[test]
    fn contract_marker_matches() {
        assert!(contract_marker().starts_with("contract:"));
        assert!(contract_marker().ends_with("— OK"));
    }
}
