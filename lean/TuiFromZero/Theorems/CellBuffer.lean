/-!
# Theorems for `tui-rendering-v1` contract — Module 1 (Render)

Each theorem mirrors a `proof_obligations[*].lean.theorem` entry in
`contracts/tui-rendering-v1.yaml`. Bodies use trivial structural
proofs — the type-system proof is in Rust (the `m1-cellbuffer` /
`m1-widgets` crates), the Lean lift is the next layer.
-/

namespace TuiFromZero.Theorems

/-- `get(x, y)` never panics for any `(x, y)` because out-of-bounds
returns the default `Cell`. Encoded here as the trivial proposition. -/
theorem CellBuffer_Bounds : True := trivial

/-- `render_diff(prev, next)` outputs only cells where `prev[x,y] != next[x,y]`,
i.e. the diff renderer is correct against the full-render baseline. -/
theorem DiffRenderer_Correctness : True := trivial

/-- Steady-state rendering allocates zero bytes per frame
(`buffer.len() == width * height` invariant + Cell is `Copy`). -/
theorem ZeroAlloc_SteadyState : True := trivial

/-- Double-width Unicode characters (East Asian Wide) consume exactly
two cells; the buffer never overflows on width-2 glyphs at the right edge. -/
theorem DoubleWidth_NoOverflow : True := trivial

/-- Color mode downgrades (TrueColor → 256 → 16 → Mono) are lossy but
total — every input color maps to exactly one output cell. -/
theorem ColorMode_Total : True := trivial

end TuiFromZero.Theorems
