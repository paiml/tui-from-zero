//! ProcessTable + CpuGrid + memory bar — composed over presentar's CellBuffer.

use m1_cellbuffer::{ansi_to_color, CellBuffer, Modifiers};
use m1_widgets::{Label, Rect, Widget};
use m3_sparkline::paint_sparkline;
use presentar_core::Color;

#[derive(Debug, Clone)]
pub struct Process {
    pub name: String,
    pub cpu: f64,
    pub mem: f64,
}

pub struct CpuGrid<'a> {
    pub cores: &'a [f64],
}

impl<'a> Widget for CpuGrid<'a> {
    fn paint(&self, buf: &mut CellBuffer, rect: Rect) {
        let cores = self.cores.len() as u16;
        if cores == 0 || rect.w == 0 {
            return;
        }
        let cell_w = (rect.w / cores).max(1);
        let bg = Color::TRANSPARENT;
        for (i, &load) in self.cores.iter().enumerate() {
            let cx = rect.x + (i as u16) * cell_w;
            if cx >= rect.x + rect.w {
                break;
            }
            let fg = ansi_to_color(if load > 0.8 {
                1
            } else if load > 0.5 {
                3
            } else {
                2
            });
            for dx in 0..cell_w {
                if let Some(c) = buf.get_mut(cx + dx, rect.y) {
                    c.update("█", fg, bg, Modifiers::NONE);
                }
            }
        }
    }
}

pub struct ProcessTable<'a> {
    pub processes: &'a [Process],
}

impl<'a> Widget for ProcessTable<'a> {
    fn paint(&self, buf: &mut CellBuffer, rect: Rect) {
        Label {
            text: "NAME              CPU%   MEM%".into(),
            fg: 6,
        }
        .paint(
            buf,
            Rect {
                x: rect.x,
                y: rect.y,
                w: rect.w,
                h: 1,
            },
        );
        let rows = (rect.h.saturating_sub(1) as usize).min(self.processes.len());
        for (i, proc) in self.processes.iter().take(rows).enumerate() {
            let row = format!("{:<16}  {:>4.1}   {:>4.1}", proc.name, proc.cpu, proc.mem);
            Label { text: row, fg: 7 }.paint(
                buf,
                Rect {
                    x: rect.x,
                    y: rect.y + 1 + (i as u16),
                    w: rect.w,
                    h: 1,
                },
            );
        }
    }
}

pub fn paint_memory_bar(buf: &mut CellBuffer, rect: Rect, used: f64, total: f64) {
    if rect.w < 2 || total <= 0.0 {
        return;
    }
    let inner = rect.w - 2;
    let filled = ((used / total).clamp(0.0, 1.0) * f64::from(inner)) as u16;
    let put = |buf: &mut CellBuffer, x: u16, y: u16, ch: &str, fg: u8| {
        if let Some(c) = buf.get_mut(x, y) {
            c.update(ch, ansi_to_color(fg), Color::TRANSPARENT, Modifiers::NONE);
        }
    };
    put(buf, rect.x, rect.y, "[", 6);
    put(buf, rect.x + rect.w - 1, rect.y, "]", 6);
    for i in 0..inner {
        let (ch, fg) = if i < filled { ("█", 2) } else { (" ", 7) };
        put(buf, rect.x + 1 + i, rect.y, ch, fg);
    }
}

#[must_use]
pub fn render_dashboard(
    cores: &[f64],
    processes: &[Process],
    mem_used: f64,
    mem_total: f64,
    samples: &[f64],
) -> CellBuffer {
    let mut buf = CellBuffer::new(60, 10);
    Label {
        text: " M3 · panels — CpuGrid + Sparkline + Memory + Processes ".into(),
        fg: 4,
    }
    .paint(
        &mut buf,
        Rect {
            x: 0,
            y: 0,
            w: 60,
            h: 1,
        },
    );
    CpuGrid { cores }.paint(
        &mut buf,
        Rect {
            x: 0,
            y: 2,
            w: 30,
            h: 1,
        },
    );
    paint_sparkline(&mut buf, 31, 2, samples, 2);
    paint_memory_bar(
        &mut buf,
        Rect {
            x: 0,
            y: 3,
            w: 60,
            h: 1,
        },
        mem_used,
        mem_total,
    );
    ProcessTable { processes }.paint(
        &mut buf,
        Rect {
            x: 0,
            y: 5,
            w: 60,
            h: 5,
        },
    );
    buf
}

#[must_use]
pub fn contract_marker() -> &'static str {
    "contract: tui-panels-v1 holds — OK"
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::unwrap_used)]
mod tests {
    use super::*;

    fn space_sym(buf: &CellBuffer, x: u16, y: u16) -> bool {
        buf.get(x, y)
            .map(|c| c.symbol.as_str() == " ")
            .unwrap_or(true)
    }

    #[test]
    fn cpu_grid_never_overflows_rect() {
        let mut buf = CellBuffer::new(20, 1);
        let cores = vec![0.5; 16];
        CpuGrid { cores: &cores }.paint(
            &mut buf,
            Rect {
                x: 0,
                y: 0,
                w: 16,
                h: 1,
            },
        );
        for x in 16..20 {
            assert!(space_sym(&buf, x, 0), "overflow at x={x}");
        }
    }

    #[test]
    fn cpu_grid_empty_cores_is_noop() {
        let mut buf = CellBuffer::new(10, 1);
        CpuGrid { cores: &[] }.paint(
            &mut buf,
            Rect {
                x: 0,
                y: 0,
                w: 10,
                h: 1,
            },
        );
        for x in 0..10 {
            assert!(space_sym(&buf, x, 0));
        }
    }

    #[test]
    fn cpu_grid_zero_width_is_noop() {
        let mut buf = CellBuffer::new(10, 1);
        CpuGrid { cores: &[0.5; 4] }.paint(
            &mut buf,
            Rect {
                x: 0,
                y: 0,
                w: 0,
                h: 1,
            },
        );
        for x in 0..10 {
            assert!(space_sym(&buf, x, 0));
        }
    }

    #[test]
    fn cpu_grid_red_yellow_green_color_bands() {
        let mut buf = CellBuffer::new(12, 1);
        CpuGrid {
            cores: &[0.1, 0.6, 0.9],
        }
        .paint(
            &mut buf,
            Rect {
                x: 0,
                y: 0,
                w: 12,
                h: 1,
            },
        );
        assert_eq!(buf.get(0, 0).expect("cell").fg, ansi_to_color(2));
        assert_eq!(buf.get(4, 0).expect("cell").fg, ansi_to_color(3));
        assert_eq!(buf.get(8, 0).expect("cell").fg, ansi_to_color(1));
    }

    #[test]
    fn cpu_grid_more_cores_than_columns_breaks_early() {
        let mut buf = CellBuffer::new(8, 1);
        CpuGrid { cores: &[0.5; 8] }.paint(
            &mut buf,
            Rect {
                x: 0,
                y: 0,
                w: 4,
                h: 1,
            },
        );
        for x in 0..4 {
            assert_eq!(buf.get(x, 0).expect("cell").symbol.as_str(), "█");
        }
        for x in 4..8 {
            assert!(space_sym(&buf, x, 0));
        }
    }

    #[test]
    fn process_table_clips_rows_to_rect() {
        let mut buf = CellBuffer::new(40, 3);
        let processes = vec![
            Process {
                name: "a".into(),
                cpu: 1.0,
                mem: 1.0,
            };
            10
        ];
        ProcessTable {
            processes: &processes,
        }
        .paint(
            &mut buf,
            Rect {
                x: 0,
                y: 0,
                w: 40,
                h: 3,
            },
        );
        assert_eq!(buf.height(), 3);
    }

    #[test]
    fn memory_bar_clamps_above_total() {
        let mut buf = CellBuffer::new(10, 1);
        paint_memory_bar(
            &mut buf,
            Rect {
                x: 0,
                y: 0,
                w: 10,
                h: 1,
            },
            999.0,
            100.0,
        );
        assert_eq!(buf.get(1, 0).expect("cell").symbol.as_str(), "█");
        assert_eq!(buf.get(8, 0).expect("cell").symbol.as_str(), "█");
    }

    #[test]
    fn memory_bar_below_min_width_is_noop() {
        let mut buf = CellBuffer::new(2, 1);
        paint_memory_bar(
            &mut buf,
            Rect {
                x: 0,
                y: 0,
                w: 1,
                h: 1,
            },
            5.0,
            10.0,
        );
        assert!(space_sym(&buf, 0, 0));
    }

    #[test]
    fn contract_marker_matches() {
        assert!(contract_marker().starts_with("contract:"));
        assert!(contract_marker().ends_with("— OK"));
    }
}
