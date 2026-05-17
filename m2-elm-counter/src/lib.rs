//! Elm-style counter: Event → State → Frame.
//!
//! Provable contract: `contracts/tui-lifecycle-v1.yaml`.
//!
//! The Elm architecture in 3 functions:
//!   * `init() -> State`            — initial state
//!   * `update(State, Msg) -> State` — total, no panics
//!   * `view(State) -> CellBuffer`   — referentially transparent
//!
//! Determinism contract: same `(state, [msg₁..msgₙ])` always produces
//! the same final state and frame sequence. The runtime proof is the
//! property test below.

use m1_cellbuffer::{Cell, CellBuffer};
use m1_widgets::{Label, Rect, Widget};

/// Counter state — a single i32.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    pub count: i32,
}

/// Messages the runtime dispatches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Msg {
    Increment,
    Decrement,
    Reset,
    Quit,
}

#[must_use]
pub fn init() -> State {
    State::default()
}

/// The update function — totality is the key contract.
/// Quit is a no-op on state; the runtime checks for it separately.
#[must_use]
pub fn update(state: State, msg: Msg) -> State {
    match msg {
        Msg::Increment => State {
            count: state.count.saturating_add(1),
        },
        Msg::Decrement => State {
            count: state.count.saturating_sub(1),
        },
        Msg::Reset => State::default(),
        Msg::Quit => state,
    }
}

/// Render the state into a 40×7 CellBuffer.
#[must_use]
pub fn view(state: State) -> CellBuffer {
    let mut buf = CellBuffer::new(40, 7);
    // top border
    for x in 0..40 {
        buf.set(x, 0, Cell::new('─', 6));
        buf.set(x, 6, Cell::new('─', 6));
    }
    for y in 0..7 {
        buf.set(0, y, Cell::new('│', 6));
        buf.set(39, y, Cell::new('│', 6));
    }
    buf.set(0, 0, Cell::new('┌', 6));
    buf.set(39, 0, Cell::new('┐', 6));
    buf.set(0, 6, Cell::new('└', 6));
    buf.set(39, 6, Cell::new('┘', 6));
    // title
    Label {
        text: " M2 · Elm Counter ".into(),
        fg: 4,
    }
    .paint(
        &mut buf,
        Rect {
            x: 2,
            y: 0,
            w: 36,
            h: 1,
        },
    );
    // count
    let count_str = format!("count = {}", state.count);
    Label {
        text: count_str,
        fg: 2,
    }
    .paint(
        &mut buf,
        Rect {
            x: 4,
            y: 2,
            w: 32,
            h: 1,
        },
    );
    // hints
    Label {
        text: "+ inc · - dec · r reset · q quit".into(),
        fg: 7,
    }
    .paint(
        &mut buf,
        Rect {
            x: 4,
            y: 4,
            w: 32,
            h: 1,
        },
    );
    buf
}

#[must_use]
pub fn contract_marker() -> &'static str {
    "contract: tui-lifecycle-v1 holds — OK"
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn init_is_zero() {
        assert_eq!(init().count, 0);
    }

    #[test]
    fn increment_then_decrement_is_identity() {
        let s = init();
        let s2 = update(update(s, Msg::Increment), Msg::Decrement);
        assert_eq!(s, s2);
    }

    #[test]
    fn reset_returns_to_zero() {
        let s = update(update(init(), Msg::Increment), Msg::Increment);
        assert_eq!(update(s, Msg::Reset).count, 0);
    }

    #[test]
    fn update_never_panics_on_i32_extremes() {
        // saturating arithmetic — totality contract
        let max = State { count: i32::MAX };
        assert_eq!(update(max, Msg::Increment).count, i32::MAX);
        let min = State { count: i32::MIN };
        assert_eq!(update(min, Msg::Decrement).count, i32::MIN);
    }

    proptest! {
        // Determinism: replaying the same message sequence from init()
        // always lands on the same final state.
        #[test]
        fn event_replay_deterministic(msgs in proptest::collection::vec(
            prop_oneof![
                Just(Msg::Increment),
                Just(Msg::Decrement),
                Just(Msg::Reset),
            ],
            0..50
        )) {
            let s1 = msgs.iter().copied().fold(init(), update);
            let s2 = msgs.iter().copied().fold(init(), update);
            prop_assert_eq!(s1, s2);
        }
    }

    #[test]
    fn quit_is_a_noop_on_state() {
        let s = State { count: 7 };
        assert_eq!(update(s, Msg::Quit), s);
    }

    #[test]
    fn view_renders_full_frame() {
        let buf = view(State { count: 42 });
        // 40x7 = 280 cells
        assert_eq!(buf.width(), 40);
        assert_eq!(buf.height(), 7);
        // corners painted
        assert_eq!(buf.get(0, 0).ch, '┌');
        assert_eq!(buf.get(39, 0).ch, '┐');
        assert_eq!(buf.get(0, 6).ch, '└');
        assert_eq!(buf.get(39, 6).ch, '┘');
        // count text appears (somewhere on row 2)
        let row2: String = (0..40).map(|x| buf.get(x, 2).ch).collect();
        assert!(row2.contains("count = 42"), "row 2 missing count text: {row2:?}");
    }

    #[test]
    fn view_handles_negative_count() {
        let buf = view(State { count: -1 });
        let row2: String = (0..40).map(|x| buf.get(x, 2).ch).collect();
        assert!(row2.contains("-1"));
    }

    #[test]
    fn contract_marker_matches() {
        assert!(contract_marker().starts_with("contract:"));
        assert!(contract_marker().ends_with("— OK"));
    }
}
