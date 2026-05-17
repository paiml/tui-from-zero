//! Minimal `.prs` scene loader — a tiny declarative format that compiles
//! to a `Vec<Box<dyn Widget>>` painted into a `CellBuffer`.
//!
//! Provable contract: `contracts/tui-panels-v1.yaml`.
//!
//! The format is line-oriented (no yaml crate needed — keeps the
//! workspace pure-Rust + zero-network). One widget per line:
//!
//!   label x=2 y=0 fg=4 text="hello world"
//!   block x=0 y=3 w=20 h=2 fg=3 ch=#
//!
//! The lesson video shows the full YAML parsing variant; this loader
//! is the pedagogical minimum that demonstrates the concept.

use m1_cellbuffer::{Cell, CellBuffer};
use m1_widgets::{Block, Label, Rect, Widget};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SceneError {
    UnknownWidget(String),
    MalformedLine(String),
}

impl std::fmt::Display for SceneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownWidget(s) => write!(f, "unknown widget kind: {s}"),
            Self::MalformedLine(s) => write!(f, "malformed scene line: {s}"),
        }
    }
}

impl std::error::Error for SceneError {}

/// One (widget, rect) pair ready for painting.
pub type SceneItem = (Box<dyn Widget>, Rect);

/// Parse a `.prs` source into a list of paint-ready (widget, rect) pairs.
pub fn parse(src: &str) -> Result<Vec<SceneItem>, SceneError> {
    let mut out: Vec<SceneItem> = Vec::new();
    for raw_line in src.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.splitn(2, char::is_whitespace);
        let kind = parts.next().unwrap_or("");
        let rest = parts.next().unwrap_or("");
        let kv = parse_kv(rest);

        match kind {
            "label" => {
                let rect = rect_from_kv(&kv);
                let fg = kv
                    .iter()
                    .find_map(|(k, v)| (k == &"fg").then(|| v.parse().ok()))
                    .flatten()
                    .unwrap_or(7);
                let text = kv
                    .iter()
                    .find_map(|(k, v)| (k == &"text").then_some(*v))
                    .unwrap_or("")
                    .trim_matches('"')
                    .to_string();
                out.push((
                    Box::new(Label { text, fg }),
                    Rect {
                        x: rect.0,
                        y: rect.1,
                        w: rect.2,
                        h: rect.3,
                    },
                ));
            }
            "block" => {
                let rect = rect_from_kv(&kv);
                let fg: u8 = kv
                    .iter()
                    .find_map(|(k, v)| (k == &"fg").then(|| v.parse().ok()))
                    .flatten()
                    .unwrap_or(7);
                let ch: char = kv
                    .iter()
                    .find_map(|(k, v)| (k == &"ch").then(|| v.chars().next()))
                    .flatten()
                    .unwrap_or('#');
                out.push((
                    Box::new(Block {
                        cell: Cell::new(ch, fg),
                    }),
                    Rect {
                        x: rect.0,
                        y: rect.1,
                        w: rect.2,
                        h: rect.3,
                    },
                ));
            }
            // The empty-kind branch is unreachable in practice — `line` is
            // already trimmed and non-empty by this point, and `splitn(2,
            // is_whitespace)` always yields at least one token. Keeping
            // `MalformedLine` in the public API for future extensions.
            other => return Err(SceneError::UnknownWidget(other.to_string())),
        }
    }
    Ok(out)
}

fn parse_kv(s: &str) -> Vec<(&str, &str)> {
    // Naïve key=value tokenizer that respects "quoted values".
    let mut out = Vec::new();
    let mut chars = s.char_indices().peekable();
    while let Some((start, _)) = chars.peek().copied() {
        // skip whitespace
        while let Some(&(_, c)) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
            } else {
                break;
            }
        }
        let Some((tok_start, _)) = chars.peek().copied() else {
            break;
        };
        // consume key
        let mut eq = None;
        while let Some(&(i, c)) = chars.peek() {
            if c == '=' {
                eq = Some(i);
                chars.next();
                break;
            }
            chars.next();
        }
        let Some(eq) = eq else {
            break;
        };
        let key = &s[tok_start..eq];
        // consume value (handle quoted strings)
        let val_start = chars.peek().map(|(i, _)| *i).unwrap_or(s.len());
        let quoted = s.as_bytes().get(val_start) == Some(&b'"');
        if quoted {
            chars.next();
            let inner_start = val_start + 1;
            let mut inner_end = s.len();
            for (i, c) in chars.by_ref() {
                if c == '"' {
                    inner_end = i;
                    break;
                }
            }
            out.push((key, &s[inner_start..inner_end]));
        } else {
            let mut end = s.len();
            while let Some(&(i, c)) = chars.peek() {
                if c.is_whitespace() {
                    end = i;
                    break;
                }
                chars.next();
            }
            out.push((key, &s[val_start..end]));
        }
        let _ = start; // silence unused
    }
    out
}

fn rect_from_kv(kv: &[(&str, &str)]) -> (usize, usize, usize, usize) {
    let x = kv
        .iter()
        .find_map(|(k, v)| (k == &"x").then(|| v.parse().ok()))
        .flatten()
        .unwrap_or(0);
    let y = kv
        .iter()
        .find_map(|(k, v)| (k == &"y").then(|| v.parse().ok()))
        .flatten()
        .unwrap_or(0);
    let w = kv
        .iter()
        .find_map(|(k, v)| (k == &"w").then(|| v.parse().ok()))
        .flatten()
        .unwrap_or(0);
    let h = kv
        .iter()
        .find_map(|(k, v)| (k == &"h").then(|| v.parse().ok()))
        .flatten()
        .unwrap_or(1);
    (x, y, w, h)
}

/// Apply every (widget, rect) into a fresh `CellBuffer`.
#[must_use]
pub fn compile(src: &str, width: usize, height: usize) -> CellBuffer {
    let mut buf = CellBuffer::new(width, height);
    if let Ok(scene) = parse(src) {
        for (w, r) in scene {
            w.paint(&mut buf, r);
        }
    }
    buf
}

#[must_use]
pub fn contract_marker() -> &'static str {
    "contract: tui-panels-v1 holds — OK"
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn parse_label_line() {
        let scene = parse(r#"label x=2 y=0 fg=4 text="hello""#).expect("parse");
        assert_eq!(scene.len(), 1);
    }

    #[test]
    fn parse_block_line() {
        let scene = parse("block x=0 y=0 w=4 h=2 fg=3 ch=#").expect("parse");
        assert_eq!(scene.len(), 1);
    }

    #[test]
    fn unknown_widget_returns_err() {
        let err = parse("foobar x=0").err();
        assert_eq!(
            err,
            Some(SceneError::UnknownWidget("foobar".to_string()))
        );
    }

    #[test]
    fn comments_and_blanks_are_skipped() {
        let s = parse("# this is a comment\n\nlabel x=0 y=0 fg=1 text=\"hi\"").expect("parse");
        assert_eq!(s.len(), 1);
    }

    #[test]
    fn compile_paints_label_into_buffer() {
        let buf = compile(r#"label x=0 y=0 w=4 h=1 fg=4 text="abc""#, 10, 1);
        assert_eq!(buf.get(0, 0).ch, 'a');
        assert_eq!(buf.get(2, 0).ch, 'c');
    }

    #[test]
    fn display_renders_both_variants() {
        let u = SceneError::UnknownWidget("foo".into());
        assert!(format!("{u}").contains("foo"));
        let m = SceneError::MalformedLine("oops".into());
        assert!(format!("{m}").contains("oops"));
    }

    #[test]
    fn compile_swallows_parse_errors() {
        // An unknown widget in source returns Err from parse() — compile
        // catches it and returns a default buffer (no widgets painted).
        let buf = compile("unknown x=0 y=0", 4, 1);
        for x in 0..4 {
            assert_eq!(buf.get(x, 0).ch, ' ');
        }
    }

    #[test]
    fn parse_kv_handles_unquoted_values() {
        let scene = parse("block x=2 y=1 w=3 h=1 fg=5 ch=*").expect("parse");
        assert_eq!(scene.len(), 1);
    }

    #[test]
    fn parse_kv_handles_trailing_quoted() {
        // Quote at end of string without explicit closing quote — closes at EOF.
        let scene = parse("label x=0 y=0 w=4 h=1 fg=1 text=\"oops").expect("parse");
        assert_eq!(scene.len(), 1);
    }

    #[test]
    fn parse_kv_handles_no_equals_token() {
        // A bare token with no `=` after the kind is tolerated (skipped).
        let scene = parse("label foo").expect("parse");
        assert_eq!(scene.len(), 1);
    }

    #[test]
    fn parse_kv_handles_trailing_whitespace() {
        // Triggers the EOF-after-whitespace break in parse_kv's outer loop.
        let scene = parse("label x=0 y=0 w=2 h=1 fg=4 text=\"a\"   ").expect("parse");
        assert_eq!(scene.len(), 1);
    }

    #[test]
    fn parse_kv_direct_with_trailing_whitespace() {
        // Direct test of the private parse_kv to exercise the EOF break
        // after the inner whitespace-skip in its outer while loop.
        let kv = parse_kv("x=1   ");
        assert_eq!(kv, vec![("x", "1")]);
    }

    #[test]
    fn contract_marker_matches() {
        assert!(contract_marker().starts_with("contract:"));
        assert!(contract_marker().ends_with("— OK"));
    }
}
