//! Probar-style snapshot test for m1-cellbuffer.
//!
//! Pattern: build a presentar `CellBuffer`, take a deterministic text
//! snapshot via `m4_tests::snapshot`, assert against a golden string.
//! This is the same pattern aprender-present-test uses for its
//! `TuiTestBackend` snapshots — adapted to our buffer types.

use m1_cellbuffer::{write_str, CellBuffer};
use m4_tests::{diff_snapshot, snapshot};

#[test]
fn probar_snapshot_two_line_label() {
    let mut buf = CellBuffer::new(12, 2);
    write_str(&mut buf, 0, 0, "render OK", 4);
    write_str(&mut buf, 0, 1, "diff = min", 2);

    // Golden snapshot — every char on every row, newline-terminated.
    let golden = "\
render OK
diff = min
";

    let mismatches = diff_snapshot(&buf, golden);
    assert!(
        mismatches.is_empty(),
        "probar snapshot diverged:\n actual:\n{}\nmismatches: {:?}",
        snapshot(&buf),
        mismatches
    );
}
