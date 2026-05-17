//! Probar-style snapshot test for m5-ptop-mini — capstone.

use m4_tests::{diff_snapshot, snapshot};
use m5_ptop_mini::{view, Snapshot};

#[test]
fn probar_snapshot_fixture_view() {
    let snap = Snapshot::fixture();
    let buf = view(&snap);
    let actual = snapshot(&buf);

    // The full dashboard contains every pillar's signature glyph/marker.
    assert!(actual.contains("CpuGrid"), "title bar mentions CpuGrid");
    assert!(actual.contains("NAME"), "ProcessTable header present");
    assert!(actual.contains("rustc"), "fixture's rustc process present");
    assert!(
        actual.contains("ptop-mini"),
        "fixture's ptop-mini process present"
    );
    assert!(actual.contains("["), "memory bar opener present");
    assert!(actual.contains("]"), "memory bar closer present");
    assert!(
        actual.contains("█"),
        "filled-cell glyph (cpu grid + memory) present"
    );

    // Determinism — same fixture renders the same bytes twice.
    let buf2 = view(&snap);
    assert!(
        diff_snapshot(&buf2, &actual).is_empty(),
        "view(fixture) is non-deterministic"
    );
}
