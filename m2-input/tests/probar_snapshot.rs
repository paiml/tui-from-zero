//! Probar-style snapshot test for m2-input — totality of key dispatch.
//!
//! Unlike the other crates, m2-input doesn't produce a CellBuffer.
//! The "snapshot" here is the dispatch-table text, captured as a
//! deterministic string and compared to a golden.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use m2_input::dispatch;

fn k(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

fn dispatch_table() -> String {
    let probes: Vec<(&str, KeyEvent)> = vec![
        ("+", k(KeyCode::Char('+'))),
        ("Up", k(KeyCode::Up)),
        ("-", k(KeyCode::Char('-'))),
        ("Down", k(KeyCode::Down)),
        ("r", k(KeyCode::Char('r'))),
        ("q", k(KeyCode::Char('q'))),
        ("Esc", k(KeyCode::Esc)),
        (
            "Ctrl+c",
            KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
        ),
        ("z", k(KeyCode::Char('z'))),
    ];
    probes
        .into_iter()
        .map(|(label, ev)| format!("{label:<8} -> {:?}\n", dispatch(ev)))
        .collect()
}

#[test]
fn probar_snapshot_dispatch_table() {
    let golden = "\
+        -> Some(Increment)
Up       -> Some(Increment)
-        -> Some(Decrement)
Down     -> Some(Decrement)
r        -> Some(Reset)
q        -> Some(Quit)
Esc      -> Some(Quit)
Ctrl+c   -> Some(Quit)
z        -> None
";
    assert_eq!(dispatch_table(), golden);
}
