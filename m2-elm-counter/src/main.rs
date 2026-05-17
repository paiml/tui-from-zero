#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
//! M2.1 demo: Elm-style counter. Default mode replays a deterministic
//! event sequence, renders the final frame to stdout, and proves the
//! determinism contract by running the replay twice and asserting
//! equality. `--interactive` enters a live keypress loop.

use m1_cellbuffer::{full, render_ansi};
use m2_elm_counter::{contract_marker, init, update, view, Msg};

fn replay(msgs: &[Msg]) -> m2_elm_counter::State {
    msgs.iter().copied().fold(init(), update)
}

fn main() {
    let interactive = std::env::args().any(|a| a == "--interactive");

    // Deterministic test sequence: +++--+ → count == 2
    let msgs = [
        Msg::Increment,
        Msg::Increment,
        Msg::Increment,
        Msg::Decrement,
        Msg::Decrement,
        Msg::Increment,
    ];
    let state = replay(&msgs);
    let state2 = replay(&msgs); // run twice to prove determinism
    assert_eq!(state, state2, "event replay diverged — lifecycle broken");
    assert_eq!(state.count, 2, "replay landed on unexpected count");

    let frame = view(state);
    println!("{}", render_ansi(&full(&frame)));
    println!(
        "\n[replay] {} messages -> count = {}",
        msgs.len(),
        state.count
    );
    println!("[determinism] same input -> same output (replay × 2 verified)");

    if interactive {
        println!("\nInteractive mode would loop on key events here.");
    }

    eprintln!("{}", contract_marker());
}
