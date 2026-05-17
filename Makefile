.DELETE_ON_ERROR:
.ONESHELL:
.SUFFIXES:

.PHONY: help install validate explain score lint audit status graph codegen \
        kani-stubs lean-stubs probar-stubs invariants scaffold generate \
        proof-status coverage demo test build fmt fmt-check clippy \
        coverage-test ci clean lean-build lean-clean

PV ?= pv
CONTRACTS := contracts/tui-rendering-v1.yaml \
             contracts/tui-lifecycle-v1.yaml \
             contracts/tui-panels-v1.yaml

help:
	@echo "tui-from-zero — pv-gated Makefile (companion repo for the tui-from-zero course)"
	@echo ""
	@echo "Every learner command goes THROUGH pv. cargo runs the demos,"
	@echo "but the contracts are the source of truth — pv validates them,"
	@echo "scores them, and emits the runtime assertions the demo binaries"
	@echo "assert against. Lean 4 proves the invariants at L5."
	@echo ""
	@echo "  Install:"
	@echo "    make install       — cargo install aprender-contracts-cli (provides pv)"
	@echo ""
	@echo "  pv contract gates (one per contract — see CONTRACTS var):"
	@echo "    make validate      — pv validate per contract  (schema gate)"
	@echo "    make explain       — pv explain per contract   (human-readable)"
	@echo "    make score         — pv score per contract     (5-dim rubric)"
	@echo "    make lint          — pv lint per contract      (validate + audit + score)"
	@echo "    make audit         — pv audit per contract"
	@echo "    make status        — pv status per contract"
	@echo "    make graph         — pv graph per contract"
	@echo ""
	@echo "  pv artifact generators (emit Rust / Lean / Kani / probar):"
	@echo "    make codegen       — pv codegen → debug_assert! statements"
	@echo "    make kani-stubs    — pv kani    → Kani proof harness stubs"
	@echo "    make lean-stubs    — pv lean    → Lean 4 theorem stubs"
	@echo "    make probar-stubs  — pv probar  → probar property test stubs"
	@echo "    make invariants    — pv invariants → type invariant trait + Kani harness"
	@echo "    make scaffold      — pv scaffold → Rust trait + test scaffolding"
	@echo "    make generate      — pv generate → all artifacts to disk"
	@echo ""
	@echo "  pv project reports:"
	@echo "    make proof-status  — pv proof-status across all contracts (L1-L5 levels)"
	@echo "    make coverage      — pv coverage report across contracts"
	@echo ""
	@echo "  Demo runs (the nine crates whose contracts pv gates):"
	@echo "    make demo          — run every demo binary (M1..M5)"
	@echo "    make test          — cargo test --workspace --release"
	@echo "    make build         — cargo build --workspace --release"
	@echo ""
	@echo "  Lean 4 proofs:"
	@echo "    make lean-build    — cd lean && lake build (type-check all theorems)"
	@echo "    make lean-clean    — cd lean && lake clean"
	@echo ""
	@echo "  Quality gates:"
	@echo "    make ci            — fmt-check + clippy + test + coverage + lint (full pre-merge)"
	@echo "    make fmt | fmt-check | clippy | coverage-test"
	@echo "    make clean         — cargo clean + remove target/pv"

# ---------------------------------------------------------------------------
# Install
# ---------------------------------------------------------------------------

install:
	@if command -v $(PV) >/dev/null 2>&1; then \
		echo "[install] pv already on PATH ($$($(PV) --version 2>&1 | head -1))"; \
	else \
		cargo install aprender-contracts-cli || exit 1; \
	fi

# ---------------------------------------------------------------------------
# pv contract gates
# ---------------------------------------------------------------------------

validate:
	@for c in $(CONTRACTS); do echo "--- pv validate $$c ---"; $(PV) validate $$c; done

explain:
	@for c in $(CONTRACTS); do echo "--- pv explain $$c ---"; $(PV) explain $$c; done

score:
	@for c in $(CONTRACTS); do echo "--- pv score $$c ---"; $(PV) score $$c; done

lint:
	@for c in $(CONTRACTS); do echo "--- pv lint $$c ---"; $(PV) lint $$c; done

audit:
	@for c in $(CONTRACTS); do echo "--- pv audit $$c ---"; $(PV) audit $$c; done

status:
	@for c in $(CONTRACTS); do echo "--- pv status $$c ---"; $(PV) status $$c; done

graph:
	@for c in $(CONTRACTS); do echo "--- pv graph $$c ---"; $(PV) graph $$c; done

# ---------------------------------------------------------------------------
# pv artifact generators
# ---------------------------------------------------------------------------

codegen:
	@mkdir -p target/pv
	@$(PV) codegen contracts/ --output target/pv/all-assertions.rs

kani-stubs:
	@mkdir -p target/pv
	@for c in $(CONTRACTS); do \
		out=target/pv/$$(basename $$c .yaml)-kani.rs; \
		echo "--- pv kani $$c -> $$out ---"; \
		$(PV) kani $$c > $$out; \
	done

lean-stubs:
	@mkdir -p target/pv/lean
	@for c in $(CONTRACTS); do \
		echo "--- pv lean $$c -> target/pv/lean/ ---"; \
		$(PV) lean $$c --output-dir target/pv/lean; \
	done

probar-stubs:
	@mkdir -p target/pv
	@for c in $(CONTRACTS); do \
		out=target/pv/$$(basename $$c .yaml)-probar.rs; \
		echo "--- pv probar $$c -> $$out ---"; \
		$(PV) probar $$c > $$out; \
	done

invariants:
	@mkdir -p target/pv
	@for c in $(CONTRACTS); do \
		out=target/pv/$$(basename $$c .yaml)-invariants.rs; \
		echo "--- pv invariants $$c -> $$out ---"; \
		$(PV) invariants $$c > $$out; \
	done

scaffold:
	@mkdir -p target/pv
	@for c in $(CONTRACTS); do \
		out=target/pv/$$(basename $$c .yaml)-scaffold.rs; \
		echo "--- pv scaffold $$c -> $$out ---"; \
		$(PV) scaffold $$c > $$out; \
	done

generate:
	@mkdir -p target/pv/generated
	@for c in $(CONTRACTS); do \
		base=$$(basename $$c .yaml); \
		echo "--- pv generate $$c -> target/pv/generated/$$base/ ---"; \
		$(PV) generate $$c --output target/pv/generated/$$base; \
	done

# ---------------------------------------------------------------------------
# pv project reports
# ---------------------------------------------------------------------------

proof-status:
	$(PV) proof-status contracts/

coverage:
	$(PV) coverage contracts/

# ---------------------------------------------------------------------------
# Demo runs — the nine crates
# ---------------------------------------------------------------------------

demo:
	@echo "=== M1 cellbuffer: render basics ==="
	@cargo run --release --bin cellbuffer-demo
	@echo "=== M1 widgets: Widget trait + Container/Row/Column ==="
	@cargo run --release --bin widgets-demo
	@echo "=== M2 elm-counter: Event -> State -> Frame ==="
	@cargo run --release --bin counter-demo
	@echo "=== M2 input: crossterm event loop ==="
	@cargo run --release --bin input-demo
	@echo "=== M3 sparkline: BrailleGraph + sparkline ==="
	@cargo run --release --bin sparkline-demo
	@echo "=== M3 panels: ProcessTable + CpuGrid ==="
	@cargo run --release --bin panels-demo
	@echo "=== M4 yaml-scene: .prs scene compiled to Widget tree ==="
	@cargo run --release --bin scene-demo
	@echo "=== M4 tests: pure-Rust TUI testing ==="
	@cargo run --release --bin tests-demo
	@echo "=== M5 ptop-mini: the capstone (single-frame smoke; use 'cargo run --release --bin ptop-mini' for the live loop) ==="
	@cargo run --release --bin ptop-mini -- --ci

test:
	PROPTEST_CASES=256 cargo test --workspace --release

build:
	cargo build --workspace --release

# ---------------------------------------------------------------------------
# Lean 4
# ---------------------------------------------------------------------------

lean-build:
	cd lean && lake build

lean-clean:
	cd lean && lake clean

# ---------------------------------------------------------------------------
# Quality gates
# ---------------------------------------------------------------------------

ci: fmt-check clippy test coverage-test lint

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

coverage-test:
	cargo llvm-cov --workspace --release --ignore-filename-regex 'main\.rs|src/bin/' --fail-under-lines 95

clean:
	cargo clean || exit 1
	rm -rf target/pv || exit 1
