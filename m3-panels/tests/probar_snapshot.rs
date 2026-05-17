//! Probar-style snapshot test for m3-panels — composed dashboard.

use m1_cellbuffer::CellBuffer;
use m3_panels::{paint_memory_bar, CpuGrid, Process, ProcessTable};
use m1_widgets::{Rect, Widget};
use m4_tests::snapshot;

#[test]
fn probar_snapshot_cpugrid_shape() {
    let mut buf = CellBuffer::new(8, 1);
    CpuGrid {
        cores: &[0.2, 0.6, 0.9, 0.1],
    }
    .paint(
        &mut buf,
        Rect {
            x: 0,
            y: 0,
            w: 8,
            h: 1,
        },
    );
    let actual = snapshot(&buf);
    // Every cell is filled with a full block.
    assert_eq!(actual, "████████\n");
}

#[test]
fn probar_snapshot_memory_bar_half_full() {
    let mut buf = CellBuffer::new(12, 1);
    paint_memory_bar(
        &mut buf,
        Rect {
            x: 0,
            y: 0,
            w: 12,
            h: 1,
        },
        5.0,
        10.0,
    );
    // [█████     ]
    let actual = snapshot(&buf);
    assert!(actual.starts_with('['));
    assert!(actual.contains("█████"));
    assert!(actual.contains("     "));
    assert!(actual.trim_end().ends_with(']'));
}

#[test]
fn probar_snapshot_process_table_header() {
    let mut buf = CellBuffer::new(32, 2);
    let processes = vec![Process {
        name: "rustc".into(),
        cpu: 88.5,
        mem: 4.6,
    }];
    ProcessTable {
        processes: &processes,
    }
    .paint(
        &mut buf,
        Rect {
            x: 0,
            y: 0,
            w: 32,
            h: 2,
        },
    );
    let actual = snapshot(&buf);
    assert!(actual.contains("NAME"));
    assert!(actual.contains("CPU%"));
    assert!(actual.contains("rustc"));
    assert!(actual.contains("88.5"));
}
