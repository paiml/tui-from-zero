#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
//! M4.2 demo: render a frame, take a snapshot, diff it against a golden,
//! report PASS/FAIL. This is the "test a TUI without Selenium" pattern.

use m1_cellbuffer::{Cell, CellBuffer};
use m1_widgets::{Container, Direction, Label, Rect, Widget};
use m4_tests::{contract_marker, diff_snapshot, snapshot};

const GOLDEN: &str = "\
┌──────────────────────────────┐
│ M4.2 · snapshot test         │
│                              │
│ render -> string -> diff     │
└──────────────────────────────┘
";

fn build_frame() -> CellBuffer {
    let mut buf = CellBuffer::new(32, 5);
    for x in 0..32 {
        buf.set(x, 0, Cell::new('─', 6));
        buf.set(x, 4, Cell::new('─', 6));
    }
    for y in 0..5 {
        buf.set(0, y, Cell::new('│', 6));
        buf.set(31, y, Cell::new('│', 6));
    }
    buf.set(0, 0, Cell::new('┌', 6));
    buf.set(31, 0, Cell::new('┐', 6));
    buf.set(0, 4, Cell::new('└', 6));
    buf.set(31, 4, Cell::new('┘', 6));
    let row = Container {
        direction: Direction::Column,
        children: vec![
            Box::new(Label {
                text: " M4.2 · snapshot test".into(),
                fg: 4,
            }),
            Box::new(Label {
                text: "".into(),
                fg: 7,
            }),
            Box::new(Label {
                text: " render -> string -> diff".into(),
                fg: 7,
            }),
        ],
    };
    row.paint(
        &mut buf,
        Rect {
            x: 1,
            y: 1,
            w: 30,
            h: 3,
        },
    );
    buf
}

fn main() {
    let buf = build_frame();
    let snap = snapshot(&buf);
    let mismatches = diff_snapshot(&buf, GOLDEN);

    println!("snapshot:");
    println!("{snap}");
    println!("diff vs golden: {} mismatch(es)", mismatches.len());
    if mismatches.is_empty() {
        println!("✓ snapshot matches golden — TUI passes test without a browser.");
    } else {
        println!("✘ mismatches: {mismatches:?}");
    }

    eprintln!("{}", contract_marker());
}
