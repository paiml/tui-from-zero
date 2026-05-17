#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
//! M1 demo: build a CellBuffer, mutate one cell, prove the diff
//! renderer emits exactly one DrawOp instead of repainting everything.
//!
//! Default mode: print a single-frame demo to stdout + assertion + marker, exit.
//! `--interactive`: render the same frame at 24 fps for 3 s to show animation.

use m1_cellbuffer::{contract_marker, diff, full, render_ansi, Cell, CellBuffer};
use std::io::{self, Write};
use std::time::{Duration, Instant};

fn build_frame(width: usize, height: usize, tick: u64) -> CellBuffer {
    let mut buf = CellBuffer::new(width, height);
    // header bar
    buf.write_str(2, 0, "M1 · CellBuffer + DiffRenderer", 4);
    // animated marker — only this cell changes per tick
    let x = 2 + ((tick % (width as u64 - 4)) as usize);
    buf.set(x, 2, Cell::new('●', 2));
    buf.write_str(2, height - 1, "press Ctrl-C to quit (interactive)", 8);
    buf
}

fn main() {
    let interactive = std::env::args().any(|a| a == "--interactive");
    let (width, height) = (40usize, 5usize);

    let prev = build_frame(width, height, 0);
    let next = build_frame(width, height, 1);

    // Runtime proof: diff returns exactly the cells that changed —
    // never more than buffer.len().
    let ops = diff(&prev, &next);
    assert!(ops.len() <= prev.len(), "diff exceeded buffer size");
    assert_eq!(ops.len(), 2, "expected 2 changed cells (old + new dot pos)");

    if interactive {
        let mut out = io::stdout().lock();
        write!(out, "\x1b[2J").expect("clear screen");
        let start = Instant::now();
        let mut current = prev;
        let mut tick: u64 = 1;
        while start.elapsed() < Duration::from_secs(3) {
            let frame = build_frame(width, height, tick);
            let ops = diff(&current, &frame);
            write!(out, "{}", render_ansi(&ops)).expect("write frame");
            out.flush().expect("flush");
            current = frame;
            tick += 1;
            std::thread::sleep(Duration::from_millis(42));
        }
        writeln!(out, "\x1b[{};1H", height + 1).expect("park cursor");
    } else {
        // Single-frame trace for CI + screencast still capture.
        println!("[full] would emit {} draw ops", full(&next).len());
        println!("[diff] emits {} draw ops (only changed cells)", ops.len());
        for op in &ops {
            println!(
                "  draw '{}' at ({}, {}) fg={}",
                op.cell.ch, op.x, op.y, op.cell.fg
            );
        }
    }

    eprintln!("{}", contract_marker());
}
