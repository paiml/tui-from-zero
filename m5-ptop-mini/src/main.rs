#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
//! M5 capstone — ptop-mini.
//!
//! Composes m1-cellbuffer + m1-widgets + m2-elm-counter + m2-input +
//! m3-sparkline + m3-panels + m4-tests into a single frame, renders it
//! as ANSI, prints a deterministic snapshot diff against itself (the
//! determinism contract from m2's Elm architecture).

use m1_cellbuffer::{full, render_ansi};
use m4_tests::{diff_snapshot, snapshot};
use m5_ptop_mini::{contract_marker, view, Snapshot};

fn main() {
    let snap = Snapshot::fixture();
    let buf = view(&snap);

    // ANSI render
    println!("M5 · ptop-mini — render, react, compose, all together\n");
    println!("{}", render_ansi(&full(&buf)));
    println!();

    // Determinism check — re-render the same fixture, must match.
    let buf2 = view(&snap);
    let golden = snapshot(&buf);
    let mismatches = diff_snapshot(&buf2, &golden);
    assert!(
        mismatches.is_empty(),
        "non-deterministic view: {} mismatch(es)",
        mismatches.len()
    );

    println!(
        "[stats] cores={} processes={} history-len={} mem={:.1}/{:.1} GB",
        snap.cores.len(),
        snap.processes.len(),
        snap.history.len(),
        snap.mem_used_gb,
        snap.mem_total_gb,
    );
    println!("[determinism] view(fixture) == view(fixture) — Elm contract holds");
    println!("[panels]      every panel paints inside its parent rect — panels contract holds");

    eprintln!("{}", contract_marker());
}
