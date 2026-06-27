use egui;
use crate::i18n::{self, Lang};
use crate::style::*;
use crate::components::{self, ColumnDef, ColumnSizing, DataTable};
use crate::barcode_scanner;

pub struct SalesState {
    pub show_sale_modal: bool,
    pub filter_date: String,
    pub form_items: Vec<SaleItemForm>,
    pub form_error: Option<String>,
    pub pagination: PaginationState,
    pub sort: SortState,
    pub barcode_input: barcode_scanner::BarcodeInput,
}

fn sort_transactions(items: &mut [monstock_core::models::Transaction], sort: &SortState) {
    let asc = sort.ascending;
    match sort.column {
        Some(0) => items.sort_by(|a, b| compare_int(a.id, b.id, asc)),
        Some(1) => items.sort_by(|a, b| compare_str(&a.timestamp, &b.timestamp, asc)),
        Some(2) => items.sort_by(|a, b| compare_float(a.total, b.total, asc)),
        _ => {}
    }
}

pub(crate) struct SaleItemForm {
    pub product_id: i32,
    pub product_name: String,
    pub quantity: String,
    pub selling_price: String,
}

impl Default for SalesState {
    fn default() -> Self {
        Self {
            show_sale_modal: false,
            filter_date: chrono::Local::now().format("%Y-%m-%d").to_string(),
            form_items: Vec::new(),
            form_error: None,
            pagination: Default::default(),
            sort: Default::default(),
            barcode_input: Default::default(),
        }
    }
}

impl components::ModalScreen for SalesState {
    fn save(&mut self, lang: Lang, conn: &mut diesel::SqliteConnection) -> bool {
        self.save(lang, conn)
    }
    fn close_modal(&mut self) {
        self.close_modal();
    }
}

impl SalesState {
    fn open_sale_modal(&mut self) {
        self.show_sale_modal = true;
        self.form_items.clear();
        self.form_error = None;
    }

    fn close_modal(&mut self) { self.show_sale_modal = false; }

    fn add_item(&mut self, id: i32, name: &str, selling_price: f64, _cost_price: f64) {
        self.form_items.push(SaleItemForm { product_id: id, product_name: name.to_string(), quantity: "1".to_string(), selling_price: format!("{:.0}", selling_price) });
    }

    fn remove_item(&mut self, index: usize) { self.form_items.remove(index); }

    fn save(&mut self, lang: Lang, conn: &mut diesel::SqliteConnection) -> bool {
        if self.form_items.is_empty() { self.form_error = Some(i18n::t("min_one_item", lang).to_string()); return false; }

        let mut items = Vec::new();
        for item in &self.form_items {
            let quantity: i32 = match item.quantity.parse() { Ok(v) if v > 0 => v, _ => { self.form_error = Some(format!("Invalid quantity for '{}'", item.product_name)); return false } };
            let selling_price: f64 = match item.selling_price.parse() { Ok(v) if v >= 0.0 => v, _ => { self.form_error = Some(format!("Invalid price for '{}'", item.product_name)); return false } };
            let product = monstock_core::services::product_service::find_by_id(conn, item.product_id);
            let cost_price = match product { Ok(Some(p)) => p.cost_price, _ => 0.0 };
            items.push(monstock_core::services::sale_service::SaleItemInput { product_id: item.product_id, product_name: item.product_name.clone(), quantity, selling_price, cost_price });
        }

        let timestamp = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        match monstock_core::services::sale_service::record_sale(conn, &timestamp, &items) {
            Ok(_) => { self.close_modal(); true }
            Err(e) => { self.form_error = Some(e.to_string()); false }
        }
    }
}

pub fn show(ui: &mut egui::Ui, conn: &mut diesel::SqliteConnection, lang: Lang, is_dark: bool, state: &mut SalesState) {
    page_header(ui, "04", i18n::t("sales", lang), i18n::t("overview", lang), is_dark);

    card(ui, is_dark, |ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(format!("{} {}", i18n::t("sales", lang), i18n::t("overview", lang))).size(14.0).color(text_color(is_dark)).strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if primary_btn(ui, &format!("+ {}", i18n::t("add", lang))).clicked() { state.open_sale_modal(); }
            });
        });
    });

    ui.add_space(8.0);
    card(ui, is_dark, |ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(i18n::t("date", lang)).size(13.0).color(text_color(is_dark)).strong());
            ui.add_space(8.0);
            ui.add(egui::TextEdit::singleline(&mut state.filter_date).desired_width(120.0));
            ui.add_space(12.0);
            let total = monstock_core::services::sale_service::daily_sales_total(conn, &state.filter_date).unwrap_or(0.0);
            amount_text(ui, &format!("{:.0} DA", total), ACCENT);
        });
    });

    ui.add_space(8.0);
    card(ui, is_dark, |ui| {
        state.pagination.total = monstock_core::services::sale_service::count_transactions_by_date(conn, &state.filter_date).unwrap_or(0);
        let (mut transactions, error) = match monstock_core::services::sale_service::find_transactions_paginated(conn, &state.filter_date, state.pagination.page, state.pagination.per_page) {
            Ok(list) => (list, None),
            Err(e) => (vec![], Some(format!("{}: {}", i18n::t("error", lang), e))),
        };
        let mut s = state.sort.clone();
        sort_transactions(&mut transactions, &s);
        DataTable::new(vec![
            ColumnDef::new("ID", ColumnSizing::Exact(60.0)),
            ColumnDef::new(i18n::t("date", lang), ColumnSizing::Remainder).resizable(true),
            ColumnDef::new(i18n::t("total", lang), ColumnSizing::Exact(100.0)),
        ], &transactions)
            .error(error.as_deref())
            .row_height(32.0)
            .show(ui, Some(&mut s), |row, tx| {
                row.col(|ui| mono_value(ui, &format!("{}", tx.id), TEXT));
                row.col(|ui| mono_value(ui, &tx.timestamp, TEXT_SEC));
                row.col(|ui| amount_text(ui, &format!("{:.0} DA", tx.total), TEXT));
            });
        state.sort = s;
        pagination_ui(ui, &mut state.pagination, is_dark);
    });

    if state.show_sale_modal {
        let title = format!("{} {}", i18n::t("add", lang), i18n::t("sales", lang));
        components::modal_window(ui.ctx(), &title, [500.0, 560.0], |ui| {
            card(ui, is_dark, |ui| {
                ui.set_min_width(ui.available_width());
                barcode_scanner::scan_ui(ui, &mut state.barcode_input);
                if state.barcode_input.take_scan().is_some() {
                    if let Ok(Some(p)) = monstock_core::services::product_service::find_by_barcode(conn, state.barcode_input.value.trim()) {
                        state.add_item(p.id, &p.name, p.selling_price, p.cost_price);
                        state.barcode_input.value.clear();
                    } else {
                        state.form_error = Some(format!("Product not found: {}", state.barcode_input.value));
                    }
                }
            });
            ui.add_space(12.0);

            ui.label(egui::RichText::new(i18n::t("available_products", lang)).size(14.0).color(text_color(is_dark)).strong());
            ui.add_space(4.0);

            let products = monstock_core::services::product_service::find_all(conn).unwrap_or_default();
            egui::ScrollArea::vertical().id_salt("product_picker").max_height(120.0).show(ui, |ui| {
                egui::Grid::new("product_picker_grid").striped(true).min_col_width(60.0).show(ui, |ui| {
                    table_header(ui, i18n::t("product", lang));
                    table_header(ui, i18n::t("stock", lang));
                    table_header(ui, i18n::t("unit_price", lang));
                    table_header(ui, "");
                    ui.end_row();

                    for p in &products {
                        ui.label(egui::RichText::new(&p.name).size(13.0).color(text_color(is_dark)));
                        let sc = if p.quantity_on_hand <= 5 { BAD } else { TEXT_SEC };
                        mono_value(ui, &format!("{}", p.quantity_on_hand), sc);
                        amount_text(ui, &format!("{:.0} DA", p.selling_price), text_color(is_dark));
                        if btn_custom(ui, egui::Button::new(egui::RichText::new("+").color(TEXT)).fill(ACCENT).corner_radius(4).min_size(egui::vec2(24.0, 20.0))).clicked() {
                            state.add_item(p.id, &p.name, p.selling_price, p.cost_price);
                        }
                        ui.end_row();
                    }
                });
            });

            ui.add_space(8.0); ui.separator(); ui.add_space(4.0);
            ui.label(egui::RichText::new(i18n::t("entered_items", lang)).size(14.0).color(text_color(is_dark)).strong());

            egui::ScrollArea::vertical().id_salt("sale_items").max_height(120.0).show(ui, |ui| {
                egui::Grid::new("sale_items_grid").striped(true).min_col_width(60.0).show(ui, |ui| {
                    table_header(ui, i18n::t("product", lang));
                    table_header(ui, i18n::t("quantity", lang));
                    table_header(ui, i18n::t("unit_price", lang));
                    table_header(ui, "");
                    ui.end_row();

                    let mut remove_idx: Option<usize> = None;
                    for (idx, item) in state.form_items.iter_mut().enumerate() {
                        ui.label(egui::RichText::new(&item.product_name).size(13.0).color(text_color(is_dark)));
                        ui.add(egui::TextEdit::singleline(&mut item.quantity).desired_width(50.0));
                        ui.add(egui::TextEdit::singleline(&mut item.selling_price).desired_width(60.0));
                        if components::delete_btn(ui, is_dark).clicked() { remove_idx = Some(idx); }
                        ui.end_row();
                    }
                    if let Some(idx) = remove_idx { state.remove_item(idx); }
                });
            });

            let err = state.form_error.clone();
            components::modal_actions(ui, lang, err.as_deref(), conn, state);
        });
    }
}
