use egui;
use egui_extras::{Column, TableBuilder, TableRow};
use crate::style::*;

/// How a column sizes itself within the table.
pub enum ColumnSizing {
    /// Fit to content width.
    Auto,
    /// Fixed width in points.
    Exact(f32),
    /// Fill remaining space equally (like `flex: 1`).
    Remainder,
}

/// Describes a single column for the `DataTable`.
pub struct ColumnDef {
    pub header: String,
    pub sizing: ColumnSizing,
    pub resizable: bool,
}

impl ColumnDef {
    pub fn new(header: impl Into<String>, sizing: ColumnSizing) -> Self {
        Self { header: header.into(), sizing, resizable: false }
    }

    pub fn resizable(mut self, v: bool) -> Self {
        self.resizable = v;
        self
    }

    fn to_egui_column(&self) -> Column {
        let mut c = match self.sizing {
            ColumnSizing::Auto => Column::auto(),
            ColumnSizing::Exact(w) => Column::exact(w),
            ColumnSizing::Remainder => Column::remainder().clip(true),
        };
        if self.resizable {
            c = c.resizable(true);
        }
        c
    }
}

/// A flexible, full-width table built on `egui_extras::Table`.
///
/// - Columns auto-size using `Auto` / `Exact` / `Remainder` strategies.
/// - Sortable headers (clickable with ▲/▼ indicators).
/// - Fixed header row while body scrolls.
/// - Optional column resizing via drag.
/// - Striped rows + consistent spacing.
pub struct DataTable<'a, T> {
    columns: Vec<ColumnDef>,
    items: &'a [T],
    error: Option<&'a str>,
    row_height: f32,
}

impl<'a, T> DataTable<'a, T> {
    pub fn new(columns: Vec<ColumnDef>, items: &'a [T]) -> Self {
        Self { columns, items, error: None, row_height: 32.0 }
    }

    pub fn error(mut self, error: Option<&'a str>) -> Self {
        self.error = error;
        self
    }

    pub fn row_height(mut self, height: f32) -> Self {
        self.row_height = height;
        self
    }

    pub fn show<F>(
        self,
        ui: &mut egui::Ui,
        mut sort: Option<&mut SortState>,
        mut render_row: F,
    )
    where
        F: FnMut(&mut TableRow, &T),
    {
        if let Some(err) = self.error {
            ui.colored_label(BAD, err);
            return;
        }

        let resizable = self.columns.iter().any(|c| c.resizable);

        let mut builder = TableBuilder::new(ui)
            .striped(true)
            .resizable(resizable);

        for col in &self.columns {
            builder = builder.column(col.to_egui_column());
        }

        let columns = &self.columns;
        let row_height = self.row_height;
        let items = self.items;

        builder
            .header(36.0, |mut header| {
                for (i, col) in columns.iter().enumerate() {
                    header.col(|ui| {
                        if let Some(ref mut s) = sort {
                            let is_active = s.column == Some(i);
                            let arrow = if is_active {
                                if s.ascending { " ▲" } else { " ▼" }
                            } else {
                                ""
                            };
                            let text = format!("{}{}", col.header, arrow);
                            let color = if is_active { TEXT } else { TEXT_SEC };
                            let r = ui.add(egui::Label::new(
                                egui::RichText::new(text).size(12.5).color(color).strong()
                            ).sense(egui::Sense::click()));
                            if r.hovered() {
                                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                            }
                            if r.clicked() {
                                if s.column == Some(i) {
                                    s.ascending = !s.ascending;
                                } else {
                                    s.column = Some(i);
                                    s.ascending = true;
                                }
                            }
                        } else {
                            ui.colored_label(TEXT_SEC, egui::RichText::new(&col.header).size(12.5).strong());
                        }
                    });
                }
            })
            .body(|mut body| {
                for item in items {
                    body.row(row_height, |mut row| {
                        render_row(&mut row, item);
                    });
                }
            });
    }
}
