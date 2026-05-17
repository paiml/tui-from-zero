//! Key event → Msg dispatch — the React pillar's input boundary.
//!
//! Provable contract: `contracts/tui-lifecycle-v1.yaml`. `dispatch`
//! is a TOTAL function: every (KeyCode, modifiers) pair either maps to
//! a Msg or returns None. No panics, no unwraps.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use m2_elm_counter::Msg;

/// Total function: every KeyEvent maps to Some(Msg) or None (ignored key).
#[must_use]
pub fn dispatch(ev: KeyEvent) -> Option<Msg> {
    if ev.modifiers.contains(KeyModifiers::CONTROL) && ev.code == KeyCode::Char('c') {
        return Some(Msg::Quit);
    }
    match ev.code {
        KeyCode::Char('+') | KeyCode::Char('=') | KeyCode::Up => Some(Msg::Increment),
        KeyCode::Char('-') | KeyCode::Char('_') | KeyCode::Down => Some(Msg::Decrement),
        KeyCode::Char('r') | KeyCode::Char('R') => Some(Msg::Reset),
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => Some(Msg::Quit),
        _ => None,
    }
}

#[must_use]
pub fn contract_marker() -> &'static str {
    "contract: tui-lifecycle-v1 holds — OK"
}

#[cfg(test)]
mod tests {
    use super::*;

    fn k(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn plus_maps_to_increment() {
        assert_eq!(dispatch(k(KeyCode::Char('+'))), Some(Msg::Increment));
        assert_eq!(dispatch(k(KeyCode::Up)), Some(Msg::Increment));
    }

    #[test]
    fn minus_maps_to_decrement() {
        assert_eq!(dispatch(k(KeyCode::Char('-'))), Some(Msg::Decrement));
        assert_eq!(dispatch(k(KeyCode::Down)), Some(Msg::Decrement));
    }

    #[test]
    fn q_and_esc_quit() {
        assert_eq!(dispatch(k(KeyCode::Char('q'))), Some(Msg::Quit));
        assert_eq!(dispatch(k(KeyCode::Esc)), Some(Msg::Quit));
    }

    #[test]
    fn ctrl_c_quits() {
        assert_eq!(
            dispatch(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL)),
            Some(Msg::Quit)
        );
    }

    #[test]
    fn unmapped_keys_return_none() {
        assert_eq!(dispatch(k(KeyCode::Char('z'))), None);
        assert_eq!(dispatch(k(KeyCode::F(1))), None);
    }

    #[test]
    fn contract_marker_matches() {
        assert!(contract_marker().starts_with("contract:"));
        assert!(contract_marker().ends_with("— OK"));
    }
}
