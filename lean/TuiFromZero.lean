import TuiFromZero.Theorems.CellBuffer
import TuiFromZero.Theorems.Lifecycle
import TuiFromZero.Theorems.Panels

/-!
# TuiFromZero — Lean 4 root

Pulls in the three per-pillar theorem modules. Each theorem mirrors a
`proof_obligations[*].lean.theorem` entry in the corresponding YAML
contract (`contracts/tui-{rendering,lifecycle,panels}-v1.yaml`).

Run `lake build` from the `lean/` directory (or `make lean-build` from
the repo root) to type-check every theorem. Build success IS the proof.
-/
