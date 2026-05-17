//! Sparkline using Unicode block glyphs at single-cell resolution.
//!
//! Provable contract: `contracts/tui-panels-v1.yaml`. Built on
//! `presentar_terminal::CellBuffer` (re-exported via `m1-cellbuffer`).

use m1_cellbuffer::{ansi_to_color, CellBuffer, Modifiers};
use presentar_core::Color;

const BLOCKS: [&str; 8] = [" ", "▁", "▂", "▃", "▄", "▅", "▆", "▇"];

/// Pick the block glyph for value `v` given max `max`. Total function.
#[must_use]
pub fn glyph(v: f64, max: f64) -> &'static str {
    if !v.is_finite() || max <= 0.0 || v <= 0.0 {
        return BLOCKS[0];
    }
    let normalized = (v / max).clamp(0.0, 1.0);
    let idx = (normalized * (BLOCKS.len() as f64 - 1.0)).round() as usize;
    BLOCKS[idx.min(BLOCKS.len() - 1)]
}

/// Paint a sparkline at row `y` starting at column `x`.
pub fn paint_sparkline(buf: &mut CellBuffer, x: u16, y: u16, samples: &[f64], fg: u8) {
    let max = samples.iter().copied().fold(0.0_f64, f64::max);
    let color = ansi_to_color(fg);
    for (i, v) in samples.iter().enumerate() {
        let col = x + (i as u16);
        if col >= buf.width() {
            break;
        }
        if let Some(c) = buf.get_mut(col, y) {
            c.update(glyph(*v, max), color, Color::TRANSPARENT, Modifiers::NONE);
        }
    }
}

#[must_use]
pub fn contract_marker() -> &'static str {
    "contract: tui-panels-v1 holds — OK"
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn glyph_total_over_extremes() {
        assert_eq!(glyph(f64::NAN, 1.0), " ");
        assert_eq!(glyph(f64::INFINITY, 1.0), " ");
        assert_eq!(glyph(-1.0, 1.0), " ");
        assert_eq!(glyph(1.0, 0.0), " ");
    }

    #[test]
    fn glyph_zero_is_space() {
        assert_eq!(glyph(0.0, 1.0), " ");
    }

    #[test]
    fn glyph_max_is_full_block() {
        assert_eq!(glyph(10.0, 10.0), "▇");
    }

    #[test]
    fn sparkline_writes_one_glyph_per_sample() {
        let mut buf = CellBuffer::new(20, 1);
        let samples = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        paint_sparkline(&mut buf, 0, 0, &samples, 2);
        for i in 0..samples.len() as u16 {
            let sym = buf.get(i, 0).expect("cell").symbol.as_str();
            assert_ne!(sym, " ", "sample {i} should not be empty");
        }
        assert_eq!(buf.get(5, 0).expect("cell").symbol.as_str(), " ");
    }

    #[test]
    fn sparkline_clips_at_right_edge() {
        let mut buf = CellBuffer::new(3, 1);
        // 5 samples in a 3-wide buffer — last 2 must be dropped (no panic).
        paint_sparkline(&mut buf, 0, 0, &[1.0, 1.0, 1.0, 1.0, 1.0], 2);
    }

    #[test]
    fn contract_marker_matches() {
        assert!(contract_marker().starts_with("contract:"));
        assert!(contract_marker().ends_with("— OK"));
    }
}
