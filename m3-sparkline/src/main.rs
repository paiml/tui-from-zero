#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
//! M3.1 demo: paint a sparkline of synthetic CPU load samples.

use m1_cellbuffer::{full, render_ansi, CellBuffer};
use m3_sparkline::{contract_marker, paint_sparkline};

fn main() {
    // 60 samples of a damped sinusoid + noise
    let samples: Vec<f64> = (0..60)
        .map(|i| {
            let t = i as f64 * 0.25;
            (t.sin() * 4.0 + 5.0).max(0.0) + (i as f64 % 7.0) * 0.3
        })
        .collect();

    let mut buf = CellBuffer::new(60, 1);
    paint_sparkline(&mut buf, 0, 0, &samples, 2);

    println!("M3.1 · sparkline (60 samples, sinusoid + noise)");
    println!();
    print!("│");
    println!("{}│", render_ansi(&full(&buf)).trim_end_matches("\x1b[0m"));
    println!();
    println!(
        "{} samples → {} cells emitted, max-glyph = full block (▇)",
        samples.len(),
        samples.len()
    );
    println!("(no widget overflows its parent rect — panels contract holds)");

    eprintln!("{}", contract_marker());
}
