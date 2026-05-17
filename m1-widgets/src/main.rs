#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
//! M1.2 demo: composite widget — a Container with 3 Labels arranged in a Row.

use m1_cellbuffer::{render_to_ansi, CellBuffer};
use m1_widgets::{contract_marker, Container, Direction, Label, Rect, Widget};

fn main() {
    let row = Container {
        direction: Direction::Row,
        children: vec![
            Box::new(Label {
                text: " RENDER".into(),
                fg: 4,
            }),
            Box::new(Label {
                text: " REACT".into(),
                fg: 5,
            }),
            Box::new(Label {
                text: " COMPOSE".into(),
                fg: 2,
            }),
        ],
    };
    let mut buf = CellBuffer::new(60, 1);
    row.paint(
        &mut buf,
        Rect {
            x: 0,
            y: 0,
            w: 60,
            h: 1,
        },
    );
    println!("{}", render_to_ansi(&buf));
    println!();
    println!("Container::Row laid out 3 Labels across 60 columns.");
    println!("Each child occupies an even slice — no child overflows its parent.");
    println!("Underlying buffer: presentar_terminal::CellBuffer (re-exported via m1-cellbuffer).");
    eprintln!("{}", contract_marker());
}
