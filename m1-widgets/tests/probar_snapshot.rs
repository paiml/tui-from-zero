//! Probar-style snapshot test for m1-widgets — Container::Row layout.

use m1_cellbuffer::CellBuffer;
use m1_widgets::{Container, Direction, Label, Rect, Widget};
use m4_tests::{diff_snapshot, snapshot};

#[test]
fn probar_snapshot_three_label_row() {
    let row = Container {
        direction: Direction::Row,
        children: vec![
            Box::new(Label {
                text: "R".into(),
                fg: 4,
            }),
            Box::new(Label {
                text: "E".into(),
                fg: 5,
            }),
            Box::new(Label {
                text: "C".into(),
                fg: 2,
            }),
        ],
    };
    let mut buf = CellBuffer::new(9, 1);
    row.paint(
        &mut buf,
        Rect {
            x: 0,
            y: 0,
            w: 9,
            h: 1,
        },
    );
    // 3 children in a width-9 row → each gets 3 cols. Label paints text
    // at top-left of its slot, leaving 2 trailing spaces per slot.
    let golden = "R  E  C  \n";
    let mismatches = diff_snapshot(&buf, golden);
    assert!(
        mismatches.is_empty(),
        "probar snapshot diverged:\n actual:\n{}\nmismatches: {:?}",
        snapshot(&buf),
        mismatches
    );
}
