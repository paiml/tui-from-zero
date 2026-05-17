//! Runtime half of the proof for tui-lifecycle-v1.
//! Exiting zero with the contract marker on stderr is the runtime check
//! that pairs with `pv validate contracts/tui-lifecycle-v1.yaml`.

fn main() {
    let marker = m2_elm_counter::contract_marker();
    eprintln!("{marker}");
}
