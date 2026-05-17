//! Probar-style snapshot test for m4-tests — the snapshot harness itself.

use m1_cellbuffer::{ansi_to_color, CellBuffer, Modifiers};
use m4_tests::{diff_snapshot, snapshot};
use presentar_core::Color;

#[test]
fn probar_snapshot_harness_roundtrips() {
    let mut buf = CellBuffer::new(4, 2);
    // Paint a custom 2-row banner using presentar_terminal cells directly.
    let fg = ansi_to_color(2);
    let bg = Color::TRANSPARENT;
    for (i, sym) in ["t", "u", "i", "*"].iter().enumerate() {
        if let Some(c) = buf.get_mut(i as u16, 0) {
            c.update(sym, fg, bg, Modifiers::NONE);
        }
    }
    for (i, sym) in ["o", "k", "!", "*"].iter().enumerate() {
        if let Some(c) = buf.get_mut(i as u16, 1) {
            c.update(sym, fg, bg, Modifiers::NONE);
        }
    }
    let s = snapshot(&buf);
    assert_eq!(s, "tui*\nok!*\n");
    assert!(
        diff_snapshot(&buf, &s).is_empty(),
        "self-diff must be empty"
    );
}
