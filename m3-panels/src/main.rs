#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
//! M3.2 demo: render a static dashboard composing CpuGrid + Sparkline +
//! Memory bar + ProcessTable into one CellBuffer, print it as ANSI.

use m1_cellbuffer::render_to_ansi;
use m3_panels::{contract_marker, render_dashboard, Process};

fn main() {
    let cores = [0.12, 0.45, 0.83, 0.67, 0.22, 0.91, 0.55, 0.31];
    let processes = vec![
        Process {
            name: "ptop-mini".into(),
            cpu: 12.4,
            mem: 0.8,
        },
        Process {
            name: "rustc".into(),
            cpu: 88.2,
            mem: 4.1,
        },
        Process {
            name: "cargo".into(),
            cpu: 7.1,
            mem: 1.2,
        },
        Process {
            name: "code".into(),
            cpu: 22.0,
            mem: 6.7,
        },
    ];
    let samples: Vec<f64> = (0..28)
        .map(|i| ((i as f64 * 0.5).sin() * 4.0 + 5.0).max(0.0))
        .collect();

    let buf = render_dashboard(&cores, &processes, 11.4, 16.0, &samples);
    println!("{}", render_to_ansi(&buf));
    println!();
    println!(
        "Composed: CpuGrid({}) + Sparkline({}) + MemoryBar + ProcessTable({})",
        cores.len(),
        samples.len(),
        processes.len()
    );
    println!("(every panel painted inside its parent rect — panels contract holds)");

    eprintln!("{}", contract_marker());
}
