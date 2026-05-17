#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
//! M3.1 demo: paint a sparkline of synthetic CPU load samples.

use m1_cellbuffer::{render_to_ansi, CellBuffer};
use m3_sparkline::{contract_marker, paint_sparkline};

fn main() {
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
    print!("{}", render_to_ansi(&buf));
    println!("│");
    println!();
    println!(
        "{} samples → painted into presentar_terminal::CellBuffer",
        samples.len()
    );
    println!("(no widget overflows its parent rect — panels contract holds)");
    eprintln!("{}", contract_marker());
}
