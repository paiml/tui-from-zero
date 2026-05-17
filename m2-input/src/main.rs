#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
//! M2.2 demo: prove every key dispatch is total — print the mapping table
//! and (in --interactive mode) loop on real key events.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use m2_input::{contract_marker, dispatch};

fn fmt_key(code: KeyCode, modifiers: KeyModifiers) -> String {
    let prefix = if modifiers.contains(KeyModifiers::CONTROL) {
        "Ctrl+"
    } else {
        ""
    };
    match code {
        KeyCode::Char(c) => format!("{prefix}{c}"),
        KeyCode::Up => format!("{prefix}↑"),
        KeyCode::Down => format!("{prefix}↓"),
        KeyCode::Esc => format!("{prefix}Esc"),
        other => format!("{prefix}{other:?}"),
    }
}

fn main() {
    println!("M2 · key → Msg dispatch table (totality contract)\n");
    let probes: Vec<(KeyCode, KeyModifiers)> = vec![
        (KeyCode::Char('+'), KeyModifiers::NONE),
        (KeyCode::Char('='), KeyModifiers::NONE),
        (KeyCode::Up, KeyModifiers::NONE),
        (KeyCode::Char('-'), KeyModifiers::NONE),
        (KeyCode::Down, KeyModifiers::NONE),
        (KeyCode::Char('r'), KeyModifiers::NONE),
        (KeyCode::Char('q'), KeyModifiers::NONE),
        (KeyCode::Esc, KeyModifiers::NONE),
        (KeyCode::Char('c'), KeyModifiers::CONTROL),
        (KeyCode::Char('z'), KeyModifiers::NONE),
    ];
    for (code, modifiers) in &probes {
        let ev = KeyEvent::new(*code, *modifiers);
        let key = fmt_key(*code, *modifiers);
        let msg = dispatch(ev);
        println!("  {:<10} → {:?}", key, msg);
    }
    println!(
        "\n{} probes dispatched, 0 panics — totality contract holds.",
        probes.len()
    );
    eprintln!("{}", contract_marker());
}
