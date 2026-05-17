//! Probar-style snapshot test for m2-elm-counter — view() output.

use m2_elm_counter::{view, State};
use m4_tests::{diff_snapshot, snapshot};

#[test]
fn probar_snapshot_count_zero_frame() {
    let buf = view(State { count: 0 });
    let actual = snapshot(&buf);

    // Sanity: corner glyphs, "count = 0" text, hint row all present.
    assert!(actual.lines().count() >= 7, "view should produce 7+ rows");
    assert!(actual.contains("┌"));
    assert!(actual.contains("┐"));
    assert!(actual.contains("└"));
    assert!(actual.contains("┘"));
    assert!(actual.contains("count = 0"));
    assert!(actual.contains("+ inc"));

    // Determinism check — view(state) must be deterministic.
    let buf2 = view(State { count: 0 });
    assert!(diff_snapshot(&buf2, &actual).is_empty());
}

#[test]
fn probar_snapshot_count_negative() {
    let buf = view(State { count: -7 });
    let actual = snapshot(&buf);
    assert!(actual.contains("count = -7"));
}
