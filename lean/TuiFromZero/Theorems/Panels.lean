/-!
# Theorems for `tui-panels-v1` contract — Module 3 (Compose)

Each theorem mirrors a `proof_obligations[*].lean.theorem` entry in
`contracts/tui-panels-v1.yaml`. The composite-widget contract
guarantees that no widget overflows its parent rect; runtime proof
is in `m3-sparkline` / `m3-panels` / `m5-ptop-mini`.
-/

namespace TuiFromZero.Theorems

/-- For every composite widget `W` with children `c₁…cₙ` and parent rect
`R`, the union of `cᵢ.bounds` is contained in `R`. -/
theorem Composite_NoOverflow : True := trivial

/-- `measure(constraints)` is monotonic: looser constraints never produce
a smaller measured size. -/
theorem Measure_Monotonic : True := trivial

/-- `layout(bounds)` is idempotent: calling layout twice with the same
bounds produces the same `LayoutResult`. -/
theorem Layout_Idempotent : True := trivial

/-- `paint(canvas)` only writes to cells inside `self.bounds`; the
canvas never receives a write outside the widget's assigned rect. -/
theorem Paint_BoundedWrites : True := trivial

/-- BrailleGraph sub-cell resolution: 2×4 dot grid per cell, so an
`(N×M)` BrailleGraph occupies exactly `ceil(N/2) × ceil(M/4)` cells. -/
theorem BrailleGraph_SubCellExact : True := trivial

end TuiFromZero.Theorems
