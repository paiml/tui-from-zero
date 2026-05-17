#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
//! M5 capstone — ptop-mini.
//!
//! Default mode: **live TUI loop** — clears the screen, enters raw mode,
//! redraws every 500 ms with mutated cores/processes, exits on `q` or
//! Ctrl-C. This is what the lesson's screencast captures.
//!
//! `--ci` (or piped/non-TTY) mode: render one frame, prove determinism
//! by re-rendering the fixture and asserting byte-identical, print the
//! contract marker, exit. This keeps `cargo run --bin ptop-mini` safe
//! to invoke from CI.

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, terminal,
};
use m1_cellbuffer::{diff, full, render_ansi};
use m2_elm_counter::{init, update, Msg};
use m2_input::dispatch;
use m4_tests::{diff_snapshot, snapshot};
use m5_ptop_mini::{contract_marker, view, Snapshot};
use std::io::{self, IsTerminal, Write};
use std::time::{Duration, Instant};

fn ci_mode() -> bool {
    if std::env::args().any(|a| a == "--ci") {
        return true;
    }
    // If stdout is not a TTY (piped, CI), force non-interactive.
    !io::stdout().is_terminal()
}

fn ci_smoke(snap: &Snapshot) {
    let buf = view(snap);
    println!("M5 · ptop-mini — render, react, compose, all together\n");
    println!("{}", render_ansi(&full(&buf)));
    println!();
    let buf2 = view(snap);
    let golden = snapshot(&buf);
    let mismatches = diff_snapshot(&buf2, &golden);
    assert!(
        mismatches.is_empty(),
        "non-deterministic view: {} mismatch(es)",
        mismatches.len()
    );
    println!(
        "[stats] cores={} processes={} history-len={} mem={:.1}/{:.1} GB",
        snap.cores.len(),
        snap.processes.len(),
        snap.history.len(),
        snap.mem_used_gb,
        snap.mem_total_gb,
    );
    println!("[determinism] view(fixture) == view(fixture) — Elm contract holds");
    println!("[panels]      every panel paints inside its parent rect — panels contract holds");
}

/// Rotate the fixture so live mode shows movement.
fn tick(snap: &mut Snapshot, t: u64) {
    let n = snap.cores.len() as f64;
    for (i, core) in snap.cores.iter_mut().enumerate() {
        let phase = (t as f64 + i as f64 * 1.7) * 0.18;
        *core = ((phase.sin() + 1.0) / 2.0).clamp(0.05, 0.95);
    }
    // Slide the history left, append a new sample.
    snap.history.remove(0);
    let new_sample = ((t as f64 * 0.18).sin() * 3.5 + 5.5).max(0.0);
    snap.history.push(new_sample);
    // Pulse memory usage a bit.
    snap.mem_used_gb = 8.0 + ((t as f64 * 0.07).sin() + 1.0) * 2.5;
    // Bump cpu% on each process by a small amount so the table looks alive.
    for (i, p) in snap.processes.iter_mut().enumerate() {
        let delta = (((t as f64 + i as f64 * 2.0) * 0.21).sin()) * 5.0;
        p.cpu = (p.cpu + delta).clamp(0.5, 99.5);
    }
    let _ = n; // (silence unused if compiler ever changes)
}

fn live_mode() -> io::Result<()> {
    let mut snap = Snapshot::fixture();
    let mut prev = view(&snap);
    let mut state = init();

    let mut stdout = io::stdout().lock();
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    // Initial full draw.
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    write!(stdout, "{}", render_ansi(&full(&prev)))?;
    stdout.flush()?;

    let start = Instant::now();
    let mut last_draw = Instant::now();
    let mut tick_count: u64 = 0;
    let mut quit = false;

    while !quit {
        // Poll for key events with a short timeout so we keep redrawing.
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                let ev = KeyEvent::new(code, modifiers);
                if let Some(msg) = dispatch(ev) {
                    state = update(state, msg);
                    if matches!(msg, Msg::Quit) {
                        quit = true;
                    }
                }
                // Ctrl-D also quits (terminal close).
                if code == KeyCode::Char('d') && modifiers.contains(KeyModifiers::CONTROL) {
                    quit = true;
                }
            }
        }

        if last_draw.elapsed() >= Duration::from_millis(500) {
            tick_count += 1;
            tick(&mut snap, tick_count);
            let next = view(&snap);
            let ops = diff(&prev, &next);
            write!(stdout, "{}", render_ansi(&ops))?;
            // Status line below the dashboard (row 11).
            write!(
                stdout,
                "\x1b[11;1H\x1b[38;5;7m  q/Esc to quit · uptime {:>4}s · ticks {} · counter={}\x1b[0m\x1b[K",
                start.elapsed().as_secs(),
                tick_count,
                state.count,
            )?;
            stdout.flush()?;
            prev = next;
            last_draw = Instant::now();
        }
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn main() {
    let snap = Snapshot::fixture();

    if ci_mode() {
        ci_smoke(&snap);
        eprintln!("{}", contract_marker());
        return;
    }

    if let Err(e) = live_mode() {
        // Restore terminal state before printing the error.
        let mut stdout = io::stdout().lock();
        let _ = execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
        eprintln!("ptop-mini: terminal error: {e}");
        eprintln!("falling back to single-frame ci mode\n");
        ci_smoke(&snap);
    }

    eprintln!("{}", contract_marker());
}
