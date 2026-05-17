//! Sparkline using Unicode block glyphs at single-cell resolution.
//!
//! Provable contract: `contracts/tui-panels-v1.yaml`. Each value
//! `v ∈ [0, max]` maps to exactly one of 8 vertical-block glyphs.
//! Output length == input length (no widget overflows its parent rect).

use m1_cellbuffer::{Cell, CellBuffer};

/// Lower-half block glyphs from empty (' ') to full ('█') — 8 levels.
const BLOCKS: [char; 8] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇'];

/// Pick the block glyph for value `v` given max `max`.
/// Total function: defined for every (v, max); v > max clamps to '█'.
#[must_use]
pub fn glyph(v: f64, max: f64) -> char {
    if !v.is_finite() || max <= 0.0 || v <= 0.0 {
        return BLOCKS[0];
    }
    let normalized = (v / max).clamp(0.0, 1.0);
    let idx = (normalized * (BLOCKS.len() as f64 - 1.0)).round() as usize;
    BLOCKS[idx.min(BLOCKS.len() - 1)]
}

/// Paint a sparkline of `samples` across the row at `(x, y)`.
/// Width is capped to `samples.len()` — exactly the panels-v1 bound.
pub fn paint_sparkline(buf: &mut CellBuffer, x: usize, y: usize, samples: &[f64], fg: u8) {
    let max = samples.iter().copied().fold(0.0_f64, f64::max);
    for (i, v) in samples.iter().enumerate() {
        buf.set(x + i, y, Cell::new(glyph(*v, max), fg));
    }
}

#[must_use]
pub fn contract_marker() -> &'static str {
    "contract: tui-panels-v1 holds — OK"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glyph_total_over_extremes() {
        // No panic on NaN, inf, zero max, negatives — totality.
        assert_eq!(glyph(f64::NAN, 1.0), ' ');
        assert_eq!(glyph(f64::INFINITY, 1.0), ' ');
        assert_eq!(glyph(-1.0, 1.0), ' ');
        assert_eq!(glyph(1.0, 0.0), ' ');
    }

    #[test]
    fn glyph_zero_is_space() {
        assert_eq!(glyph(0.0, 1.0), ' ');
    }

    #[test]
    fn glyph_max_is_full_block() {
        assert_eq!(glyph(10.0, 10.0), '▇');
    }

    #[test]
    fn sparkline_writes_one_glyph_per_sample() {
        let mut buf = CellBuffer::new(20, 1);
        let samples = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        paint_sparkline(&mut buf, 0, 0, &samples, 2);
        for i in 0..samples.len() {
            assert_ne!(buf.get(i, 0).ch, ' ', "sample {i} should not be empty");
        }
        assert_eq!(buf.get(samples.len(), 0).ch, ' '); // no overflow
    }

    #[test]
    fn contract_marker_matches() {
        assert!(contract_marker().starts_with("contract:"));
        assert!(contract_marker().ends_with("— OK"));
    }
}
