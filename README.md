# tui-from-zero

Companion repository for the **TUI from Zero** Coursera course — the next course in the
[Rust for Data Engineering](https://www.coursera.org/specializations/rust-for-data-engineering)
specialization, built around [`presentar`](https://github.com/paiml/aprender) — the pure-Rust
TUI framework from `aprender`.

In five weeks the learner builds a 200-line `ptop` clone, every rung up the stack — cell
buffer → widget → event loop → declarative scene → composed app — gated by three named
YAML contracts that [`pv`](https://github.com/paiml/aprender/tree/main/crates/aprender-contracts-cli)
validates and that Lean 4 proves the invariants of.

## The `pv` workflow

**Every demo in this repo is gated by `pv`.** The YAML contracts in [`contracts/`](contracts/)
are the single source of truth; the nine Rust crates are the implementations `pv` scores them
against; the Lean 4 modules in [`lean/`](lean/) discharge the universal claim.

```bash
make install      # cargo install aprender-contracts-cli
make validate     # pv validate every contract — schema gate
make score        # pv score every contract  — 5-dim rubric grade
make lint         # pv lint   every contract — validate + audit + score
make demo         # run the 9 demo binaries pv's contracts gate
make lean-build   # cd lean && lake build  — type-check every theorem (L5 proof)
make ci           # fmt + clippy + test + 100% cov + pv lint
```

See `make help` for the full list.

## The three pillars

| Pillar | Contract | What it proves | Demo crates |
|---|---|---|---|
| **Render** | [`tui-rendering-v1.yaml`](contracts/tui-rendering-v1.yaml) | Diff renderer never emits more cells than the buffer holds; double-width chars never overflow; steady-state rendering is zero-alloc | [`m1-cellbuffer`](m1-cellbuffer/) [`m1-widgets`](m1-widgets/) [`m4-tests`](m4-tests/) |
| **React** | [`tui-lifecycle-v1.yaml`](contracts/tui-lifecycle-v1.yaml) | Elm-style: every Event maps to exactly one State, every State maps to exactly one Frame; event replay is deterministic | [`m2-elm-counter`](m2-elm-counter/) [`m2-input`](m2-input/) |
| **Compose** | [`tui-panels-v1.yaml`](contracts/tui-panels-v1.yaml) | Composite widgets satisfy the measure/layout contract — no widget overflows its parent rect; BrailleGraph sub-cell math is exact | [`m3-sparkline`](m3-sparkline/) [`m3-panels`](m3-panels/) [`m4-yaml-scene`](m4-yaml-scene/) [`m5-ptop-mini`](m5-ptop-mini/) |

Each contract declares a formula, domain, codomain, invariants, proof obligations,
falsification tests, and a Kani harness stub. `pv validate` enforces the schema; `pv score`
grades each contract across five dimensions (Spec / Falsify / Kani / Lean / Bind); Lean 4
discharges the universal claim at L5; the demo binaries assert the runtime half of the proof.

## Demos

| Crate | Lesson coverage | Gating contract | Demo binary |
|---|---|---|---|
| [`m1-cellbuffer`](m1-cellbuffer/) | M1.1 cell buffer + diff renderer | `tui-rendering-v1` | `cellbuffer-demo` |
| [`m1-widgets`](m1-widgets/) | M1.2 Widget trait + Container/Row/Column | `tui-rendering-v1` | `widgets-demo` |
| [`m2-elm-counter`](m2-elm-counter/) | M2.1 Elm architecture in 80 lines | `tui-lifecycle-v1` | `counter-demo` |
| [`m2-input`](m2-input/) | M2.2 crossterm event loop + key bindings | `tui-lifecycle-v1` | `input-demo` |
| [`m3-sparkline`](m3-sparkline/) | M3.1 BrailleGraph + sparkline | `tui-panels-v1` | `sparkline-demo` |
| [`m3-panels`](m3-panels/) | M3.2 ProcessTable + CpuGrid | `tui-panels-v1` | `panels-demo` |
| [`m4-yaml-scene`](m4-yaml-scene/) | M4.1 `.prs` YAML-driven scene | `tui-panels-v1` | `scene-demo` |
| [`m4-tests`](m4-tests/) | M4.2 pure-Rust TUI test harness | `tui-rendering-v1` | `tests-demo` |
| [`m5-ptop-mini`](m5-ptop-mini/) | M5 capstone — 200-LOC ptop clone | `tui-panels-v1` | `ptop-mini` |

## The four pillars (of how this is taught)

1. **Render** — Cells, escapes, diff. The terminal is a grid. (M1)
2. **React** — Elm-style: Event → State → Diff → Draw. (M2)
3. **Compose** — Widgets, panels, layout. (M3)
4. **Declare + Verify** — YAML scenes + pure-Rust testing. (M4)

The capstone (M5) composes all four into a working `ptop-mini`.

## Prerequisites

- Rust 1.75+ (`rustup default stable`)
- `aprender-contracts-cli` for `pv` (the contract validator + scorer)
- Optional: `elan` + Lean 4 toolchain for `make lean-build`
- Optional: `cargo-llvm-cov` for the 100% line-coverage gate

```bash
make install
```

## Quick start

```bash
git clone https://github.com/paiml/tui-from-zero
cd tui-from-zero
make install

# Gate the contracts (schema + rubric)
make validate      # 0 errors per contract
make score         # prints the 5-dim rubric per contract

# Build + run the demos pv's contracts gate
make build
make demo

# Type-check the Lean proofs
make lean-build

# Full pre-merge gate (fmt + clippy + test + 100% cov + pv lint)
make ci
```

## Repository layout

```
contracts/
  tui-rendering-v1.yaml      ← M1 + M4-tests render contract
  tui-lifecycle-v1.yaml      ← M2 lifecycle contract
  tui-panels-v1.yaml         ← M3 + M4-scene + M5 panels contract
m1-cellbuffer/               ← cell buffer + diff renderer
m1-widgets/                  ← Widget trait + Container/Row/Column
m2-elm-counter/              ← Elm-style counter
m2-input/                    ← crossterm event loop
m3-sparkline/                ← BrailleGraph + sparkline
m3-panels/                   ← ProcessTable + CpuGrid
m4-yaml-scene/               ← .prs YAML-driven scene
m4-tests/                    ← pure-Rust TUI test harness
m5-ptop-mini/                ← the 200-LOC capstone
lean/                        ← Lean 4 proofs of the contract invariants
.github/workflows/ci.yml     ← pv-gated CI
Makefile                     ← all pv subcommands + quality gates
```

## License

Dual-licensed under MIT or Apache-2.0 — pick the one that fits your downstream use.
SPDX: `MIT OR Apache-2.0`.
