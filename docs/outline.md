# TUI From Zero — Course Outline

**Duration**: 5 weeks
**Slot**: TBD in the [Rust for Data Engineering specialization](https://www.coursera.org/specializations/rust-for-data-engineering) (proposed c8 or c22)
**Companion repo**: this repository
**Author**: paiml

## Overview

Five weeks of pure-Rust terminal UI built on `aprender-present-terminal`. The learner walks every rung of the stack — cell buffer → widget → event loop → declarative scene → composed app — gated by three named YAML contracts (`pv`) and Lean 4 theorems. The capstone is a 200-LOC-class `ptop` clone running over the production `presentar_terminal::CellBuffer`.

## Learning Outcomes

1. Render text + colour into a `CellBuffer`, emitting only the cells that changed each frame via `DiffRenderer`.
2. Compose `Widget` trees in the `Container`/`Row`/`Column` idiom, painting them into a presentar buffer without ever overflowing the parent rect.
3. Drive a TUI in the Elm style (`init` / `update` / `view`), proving event-replay determinism with a proptest harness.
4. Test TUIs without a browser using probar-style snapshot diffs against a golden cell buffer.
5. Compose every prior module into a live `ptop`-style dashboard powered by `crossterm` + presentar.

---

## Module 1: Render — CellBuffer + DiffRenderer

Covers the foundation of every TUI: terminal as a 2D grid of `Cell`s (symbol + fg + bg + modifiers); double-width Unicode safety; the diff renderer that emits only the cells that changed between frames; the Widget trait + composite pattern (`Container`/`Row`/`Column`/`Block`/`Label`).

**Gating contract**: [`tui-rendering-v1`](../contracts/tui-rendering-v1.yaml)
**Lean theorems**: `Theorems.CellBuffer_Bounds`, `DiffRenderer_Correctness`, `ZeroAlloc_SteadyState`, `DoubleWidth_NoOverflow`, `ColorMode_Total`
**Learning objectives**: Remember terminal cell anatomy · Understand the diff-render contract · Apply Widget composition

### Lesson 1.1: CellBuffer + DiffRenderer

| File | Title | Type |
|------|-------|------|
| 1.1.1-key-terms.md | Key Terms: Cells, Buffers, Diffs | Reading |
| 1.1.1-the-terminal-is-a-grid.mp4 | The terminal is a grid of cells | Video |
| 1.1.2-presentar-cellbuffer.mp4 | `presentar_terminal::CellBuffer` — anatomy | Video |
| 1.1.3-diffrenderer-emits-only-changes.mp4 | DiffRenderer: emit only what changed | Video |
| 1.1-reflection.md | Reflection: Render Pillar | Reading |

### Lesson 1.2: Widget trait + Composite pattern

| File | Title | Type |
|------|-------|------|
| 1.2.1-key-terms.md | Key Terms: Widget, Container, Row, Column | Reading |
| 1.2.1-the-widget-trait.mp4 | The Widget trait — one method, paint(rect) | Video |
| 1.2.2-container-row-column.mp4 | Composite: Container/Row/Column | Video |
| 1.2.3-block-and-label.mp4 | Leaf widgets: Block + Label | Video |
| 1.2-reflection.md | Reflection: Composition without overflow | Reading |
| 1-ungraded-check.md | Module 1 ungraded check (3 questions) | Practice |

---

## Module 2: React — Elm-style Event Loop

Covers the React pillar: Elm-style architecture (`init` / `update` / `view`), event-replay determinism, crossterm `KeyEvent` → `Msg` dispatch.

**Gating contract**: [`tui-lifecycle-v1`](../contracts/tui-lifecycle-v1.yaml)
**Lean theorems**: `Update_Total`, `View_Pure`, `EventReplay_Deterministic`, `EventLoop_TerminatesOnQuit`
**Learning objectives**: Apply the Elm architecture · Analyze update totality · Evaluate event-replay determinism

### Lesson 2.1: Elm Architecture

| File | Title | Type |
|------|-------|------|
| 2.1.1-key-terms.md | Key Terms: init, update, view, Msg | Reading |
| 2.1.1-init-update-view.mp4 | `init() -> State` / `update(s, m) -> State` / `view(s) -> CellBuffer` | Video |
| 2.1.2-counter-app.mp4 | The Elm counter: 80 lines of Rust | Video |
| 2.1.3-replay-determinism.mp4 | Replay determinism via proptest | Video |
| 2.1-reflection.md | Reflection: Same input → same output | Reading |

### Lesson 2.2: crossterm Input

| File | Title | Type |
|------|-------|------|
| 2.2.1-key-terms.md | Key Terms: KeyEvent, KeyModifiers, dispatch totality | Reading |
| 2.2.1-crossterm-event-loop.mp4 | crossterm's event loop | Video |
| 2.2.2-keyevent-to-msg.mp4 | Total `dispatch(KeyEvent) -> Option<Msg>` | Video |
| 2.2.3-ctrl-c-and-quit.mp4 | Quit handling: Esc, Ctrl-C, q | Video |
| 2.2-reflection.md | Reflection: Totality at the input boundary | Reading |
| 2-ungraded-check.md | Module 2 ungraded check (3 questions) | Practice |

---

## Module 3: Compose — Widgets in Anger

Covers the production widgets: BrailleGraph / sparkline glyphs, CpuGrid + ProcessTable + MemoryBar, the panels contract (no widget overflows its parent rect).

**Gating contract**: [`tui-panels-v1`](../contracts/tui-panels-v1.yaml)
**Lean theorems**: `Composite_NoOverflow`, `Measure_Monotonic`, `Layout_Idempotent`, `Paint_BoundedWrites`, `BrailleGraph_SubCellExact`
**Learning objectives**: Apply composite widgets to real-time data · Evaluate panel layout · Create a CpuGrid dashboard

### Lesson 3.1: Charts — Sparkline + BrailleGraph

| File | Title | Type |
|------|-------|------|
| 3.1.1-key-terms.md | Key Terms: sparkline, block-glyph, BrailleGraph | Reading |
| 3.1.1-block-glyph-sparkline.mp4 | The 8-level block-glyph sparkline | Video |
| 3.1.2-braillegraph-sub-cell.mp4 | BrailleGraph at 2×4 sub-cell resolution | Video |
| 3.1.3-totality-of-glyph.mp4 | Totality: NaN, Inf, negative, zero-max | Video |
| 3.1-reflection.md | Reflection: Glyphs as a discrete code | Reading |

### Lesson 3.2: Panels — CpuGrid + ProcessTable

| File | Title | Type |
|------|-------|------|
| 3.2.1-key-terms.md | Key Terms: CpuGrid, ProcessTable, MemoryBar, parent rect | Reading |
| 3.2.1-cpu-grid.mp4 | CpuGrid: one cell per core, threshold colors | Video |
| 3.2.2-process-table.mp4 | ProcessTable: a row per pid, formatted columns | Video |
| 3.2.3-memory-bar.mp4 | MemoryBar gauge — [████      ] | Video |
| 3.2-reflection.md | Reflection: Composing 4 panels into 1 frame | Reading |
| 3-ungraded-check.md | Module 3 ungraded check (3 questions) | Practice |

---

## Module 4: Declare + Verify — YAML Scenes and Probar Tests

Covers the `.prs` declarative scene format (parse a YAML-shaped DSL into a Widget tree) and probar-style snapshot testing (render to a buffer, diff against an inline golden — no Selenium, no headless terminal).

**Gating contracts**: [`tui-panels-v1`](../contracts/tui-panels-v1.yaml) (scenes) + [`tui-rendering-v1`](../contracts/tui-rendering-v1.yaml) (tests)
**Lean theorems**: `Composite_NoOverflow`, `Paint_BoundedWrites`, `DiffRenderer_Correctness`
**Learning objectives**: Create declarative scenes · Test TUIs without a browser

### Lesson 4.1: .prs YAML-Driven Scenes

| File | Title | Type |
|------|-------|------|
| 4.1.1-key-terms.md | Key Terms: .prs format, declarative widget, parse | Reading |
| 4.1.1-why-declarative-tui.mp4 | Why declarative TUI? | Video |
| 4.1.2-prs-format.mp4 | The .prs format: label/block lines | Video |
| 4.1.3-compile-to-widget-tree.mp4 | Compile to a Widget tree, paint into a buffer | Video |
| 4.1-reflection.md | Reflection: UI as data | Reading |

### Lesson 4.2: Probar Snapshot Testing

| File | Title | Type |
|------|-------|------|
| 4.2.1-key-terms.md | Key Terms: snapshot, golden, diff_snapshot | Reading |
| 4.2.1-snapshot-as-string.mp4 | Stringify a CellBuffer into a deterministic snapshot | Video |
| 4.2.2-golden-diff.mp4 | Golden diff vs inline string | Video |
| 4.2.3-probar-pattern.mp4 | aprender-present-test (probar) snapshot pattern | Video |
| 4.2-reflection.md | Reflection: TUI tests without a browser | Reading |
| 4-ungraded-check.md | Module 4 ungraded check (3 questions) | Practice |

---

## Module 5: Capstone — Build ptop-mini

Compose every prior module into a working ptop-style terminal dashboard.

**Gating contract**: [`tui-panels-v1`](../contracts/tui-panels-v1.yaml)
**Learning objectives**: Create a ptop-mini · Evaluate contract scores · Compose all 4 pillars

### Lesson 5.1: Composing the Pillars

| File | Title | Type |
|------|-------|------|
| 5.1.1-key-terms.md | Key Terms: capstone, Snapshot fixture, view function | Reading |
| 5.1.1-snapshot-fixture.mp4 | The Snapshot fixture: deterministic input | Video |
| 5.1.2-view-composes.mp4 | view(snapshot) composes m1+m2+m3+m4 | Video |
| 5.1.3-live-loop-vs-ci.mp4 | Live crossterm loop vs --ci smoke mode | Video |
| 5.1-reflection.md | Reflection: Three pillars, one frame | Reading |

### Lesson 5.2: Demo — ptop-mini live

| File | Title | Type |
|------|-------|------|
| 5.2.1-key-terms.md | Key Terms: live loop, raw mode, alt screen | Reading |
| 5.2.1-demo-ptop-mini.mp4 | Demo: ptop-mini running over /proc fixtures | Video |
| 5.2-reflection.md | Reflection: From zero to ptop in 5 weeks | Reading |
| course-graded-quiz.md | **Graded quiz** (5 questions, 80% pass) | Graded |

---

## Total

| Section | Count |
|---|---|
| Modules | 5 |
| Lessons | 10 |
| Key-terms readings | 10 |
| Reflections | 10 |
| Videos | 22 (4-6 min each) |
| Ungraded checks | 4 (one per Module 1-4) |
| Graded quiz | 1 (end of Module 5, 5Q @ 80%) |
| **Total items** | **57** |

Roughly 4-5 hours of learner content, per the PAIML Course Design Standard.

## Calibration

Conforms to `docs/paiml-course-design-standard.md` in `paiml/course-studio`:
- Every reading ≤ 150 words / 1 minute
- Every video ≤ 6 minutes
- One ungraded check per module (3 questions, end-of-module placement)
- Exactly one graded quiz at course end (5 questions, 80% pass)
- ❌ No AI Coach / Dialogue / Role Play / module-level graded quizzes / "Before You Go" readings
