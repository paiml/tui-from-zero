//! Probar-style snapshot test for m3-sparkline — Unicode block glyphs.

use m1_cellbuffer::CellBuffer;
use m3_sparkline::paint_sparkline;
use m4_tests::diff_snapshot;

#[test]
fn probar_snapshot_ramp_samples() {
    let mut buf = CellBuffer::new(8, 1);
    // 8 samples from 0..max — should produce a clean ramp of block glyphs.
    let samples: Vec<f64> = (0..8).map(|i| i as f64).collect();
    paint_sparkline(&mut buf, 0, 0, &samples, 2);
    // glyph(0,7)=" " glyph(1,7)≈"▁" ... glyph(7,7)="▇"
    let golden = " ▁▂▃▄▅▆▇\n";
    let mismatches = diff_snapshot(&buf, golden);
    assert!(
        mismatches.is_empty(),
        "probar snapshot diverged: {mismatches:?}"
    );
}
