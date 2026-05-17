//! Runtime half of the proof for tui-panels-v1.
//! Exiting zero with the contract marker on stderr is the runtime check
//! that pairs with `pv validate contracts/tui-panels-v1.yaml`.

fn main() {
    let marker = m3_sparkline::contract_marker();
    eprintln!("{marker}");
}
