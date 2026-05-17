# Demo Script — ptop-mini (Lesson 5.2.1)

**Target length**: 5-6 minutes
**Audio mode**: screencast (`content_mode = "demo"`)
**Format**: command on screen → ≤ 1-line annotation per beat

The capstone walkthrough composes every prior module into one running TUI. The script is written in the terse "command + 1-line annotation" style so the recorded demo and the SRT line up cleanly without trimming.

---

## Beat 1 — Frame the goal

```
clear
echo "Five weeks. Three pillars. Zero JavaScript. One ptop."
```

> `ptop-mini` reuses *every* widget you wrote: `CellBuffer`, `Container`, the Elm loop, `Sparkline`, `CpuGrid`, `ProcessTable`, the `.prs` scene loader, the probar harness. Today we run it.

## Beat 2 — Show the workspace

```
ls m5-ptop-mini/src
```

> One library (the `view(snapshot)` pure function), one binary (the live `crossterm` loop + `--ci` smoke path), one probar snapshot test.

```
bat m5-ptop-mini/src/lib.rs | head -40
```

> `view(snapshot) -> CellBuffer` is **pure**. No I/O, no clock, no `/proc`. The fixture (the `Snapshot` struct) is the only input.

## Beat 3 — Smoke test in CI mode

```
cargo run -p m5-ptop-mini -- --ci
```

> `--ci` paints exactly one frame and exits. Same code path as the live loop, just gated by `io::stdout().is_terminal()`. This is how the GitHub Actions runner exercises the capstone.

## Beat 4 — Show the snapshot fixture

```
bat m5-ptop-mini/src/snapshot.rs
```

> The fixture is deterministic: 8 cores at fixed loads, 5 processes with fixed memory, 60 sparkline samples in a fixed window. Same input → same buffer → same probar snapshot.

## Beat 5 — Run probar against the fixture

```
cargo test -p m5-ptop-mini -- --nocapture snapshot_matches
```

> One test. It paints `view(Snapshot::fixture())` into a `TuiTestBackend`, stringifies the cells, diffs against the inline golden. Zero terminal needed, zero browser needed, zero GUI needed — this IS TUI testing.

## Beat 6 — Live loop over /proc

```
cargo run -p m5-ptop-mini --release
```

> Same `view` function, real `/proc` snapshot. Raw mode + alt screen + `crossterm::poll(50ms)` → diff-render only the cells that changed. ESC, `q`, or Ctrl-C cleans up.

> ⌨️  Press `q` → graceful exit, terminal restored.

## Beat 7 — Walk the contracts

```
pv list contracts/
```

> Three named YAML contracts. Each one names obligations our code must satisfy:
> - `tui-rendering-v1` — CellBuffer bounds, diff correctness, zero-alloc steady state
> - `tui-lifecycle-v1` — update totality, view purity, event-replay determinism
> - `tui-panels-v1` — no widget overflows its parent rect, layout is idempotent

```
pv proof-status contracts/
```

> All three contracts: **Spec ✓ · Falsify ✓ · Kani ✓ · Lean ✓ · Bind ✓** — full L5 verification ladder.

## Beat 8 — One Lean theorem in anger

```
bat lean/TuiFromZero/Theorems/CellBuffer.lean | head -30
```

> Open `cellbuffer_set_bounded`: every write to a CellBuffer is provably within the (width, height) rect. The theorem and the test from beat 5 prove the same fact at two levels — Lean for the type, probar for the executable.

## Beat 9 — Score it

```
pv score --strict contracts/
```

> Five dimensions × three contracts. The amber `aprender_presentar + probar` strip on the hero is not decoration; it's the underlying platform every column points at.

## Beat 10 — Pop the alt-screen, close out

```
echo "Course: tui-from-zero. Tech: aprender-present-* on crates.io. License: MIT/Apache-2.0."
```

> Five weeks ago, an empty `CellBuffer`. Today, a working terminal dashboard with a Lean-verified core and a probar test you can run in CI. **From zero. To ptop.**

---

## Production notes (record-time)

- **Terminal size**: 120×40, scrollback OFF.
- **Font**: JetBrains Mono 18 pt, line-height 1.2.
- **Theme**: dark `#0d1117` background, foreground `#c9d1d9`, accent `#4a9eff`.
- **Pacing**: 12-18 sec per beat; total run-time ≤ 6 min hard cap (endcard rule).
- **Keystroke cadence**: 1 keypress per ~80 ms during typed lines (use `asciinema` if recording the keystrokes live).
- **Cut points**: after each `cargo` invocation prints its first line of output — never wait for the full compile.
- **Endcard**: `ptop-mini — five weeks, three pillars, zero JS` (title `ptop-mini`, subtitle `five weeks, three pillars, zero JS` — joint budget under the 91-char F-cap).
