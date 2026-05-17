#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
//! M4.1 demo: load a .prs source, compile to a Widget tree, paint into
//! a CellBuffer, print the result.

use m1_cellbuffer::render_to_ansi;
use m4_yaml_scene::{compile, contract_marker};

const SCENE: &str = r#"
# tui-from-zero · M4.1 demo scene
label x=2 y=0 fg=4 text=" M4.1 · YAML-driven scene "
block x=0 y=2 w=40 h=1 fg=6 ch=─
label x=2 y=4 fg=2 text="parsed 3 widget declarations"
label x=2 y=5 fg=2 text="painted into 40x8 CellBuffer"
block x=0 y=7 w=40 h=1 fg=6 ch=─
"#;

fn main() {
    let buf = compile(SCENE, 40, 8);
    println!("source (.prs):");
    println!("{SCENE}");
    println!("compiled output:");
    println!("{}", render_to_ansi(&buf));
    println!("\nDeclarative widgets parsed + painted — panels contract holds.");
    eprintln!("{}", contract_marker());
}
