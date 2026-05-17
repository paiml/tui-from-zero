//! Runtime half of the proof for tui-rendering-v1.
//! Exiting zero with the contract marker on stderr is the runtime check
//! that pairs with `pv validate contracts/tui-rendering-v1.yaml`.

fn main() {
    let marker = m4_tests::contract_marker();
    eprintln!("{marker}");
}
