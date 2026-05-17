//! Probar-style snapshot test for m4-yaml-scene — declarative .prs compile.

use m4_tests::snapshot;
use m4_yaml_scene::compile;

#[test]
fn probar_snapshot_scene_compiles_to_expected_layout() {
    let src = r#"
label x=0 y=0 w=5 h=1 fg=4 text="hi!"
block x=0 y=1 w=5 h=1 fg=3 ch=#
"#;
    let buf = compile(src, 5, 2);
    let actual = snapshot(&buf);
    assert_eq!(actual, "hi!  \n#####\n");
}
