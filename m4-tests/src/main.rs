#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
//! M4.2 demo: render a frame, take a snapshot, diff against a golden, report PASS/FAIL.

use m1_cellbuffer::{ansi_to_color, CellBuffer, Modifiers};
use m1_widgets::{Container, Direction, Label, Rect, Widget};
use m4_tests::{contract_marker, diff_snapshot, snapshot};
use presentar_core::Color;

const GOLDEN: &str = "\
┌──────────────────────────────┐
│ M4.2 · snapshot test         │
│                              │
│ render -> string -> diff     │
└──────────────────────────────┘
";

fn paint_border(buf: &mut CellBuffer) {
    let (w, h) = (buf.width(), buf.height());
    let fg = ansi_to_color(6);
    let bg = Color::TRANSPARENT;
    let put = |buf: &mut CellBuffer, x: u16, y: u16, ch: &str| {
        if let Some(c) = buf.get_mut(x, y) {
            c.update(ch, fg, bg, Modifiers::NONE);
        }
    };
    for x in 0..w {
        put(buf, x, 0, "─");
        put(buf, x, h - 1, "─");
    }
    for y in 0..h {
        put(buf, 0, y, "│");
        put(buf, w - 1, y, "│");
    }
    put(buf, 0, 0, "┌");
    put(buf, w - 1, 0, "┐");
    put(buf, 0, h - 1, "└");
    put(buf, w - 1, h - 1, "┘");
}

fn build_frame() -> CellBuffer {
    let mut buf = CellBuffer::new(32, 5);
    paint_border(&mut buf);
    let col = Container {
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
    col.paint(
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
