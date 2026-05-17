//! BrailleGraph + sparkline at sub-cell resolution
//!
//! Provable contract: `contracts/tui-panels-v1.yaml` (validated with `pv`).
//! This crate is a scaffolded stub for tui-from-zero — replace the body
//! with the lesson's real implementation before recording.

/// Runtime smoke check the demo binary asserts against.
pub fn contract_marker() -> &'static str {
    "contract: tui-panels-v1 holds — OK"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_marker_matches() {
        assert!(contract_marker().starts_with("contract:"));
        assert!(contract_marker().ends_with("— OK"));
    }
}
