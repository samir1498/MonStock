use egui;
use crate::i18n::{self, Lang};
use crate::style::*;
use crate::components;

#[derive(Default)]
pub struct ProductsState {
    pub show_modal: bool,
    pub editing_id: Option<i32>,
    pub form_name: String,
    pub form_barcode: String,
    pub form_cost_price: String,
    pub form_selling_price: String,
    pub form_error: Option<String>,
    pub pagination: PaginationState,
    pub sort: SortState,
}

fn sort_products(items: &mut [monstock_core::models::Product], sort: &SortState) {
    let asc = sort.ascending;
    match sort.column {
        Some(0) => items.sort_by(|a, b| compare_str(&a.name, &b.name, asc)),
        Some(1) => items.sort_by(|a, b| compare_str(a.barcode.as_deref().unwrap_or(""), b.barcode.as_deref().unwrap_or(""), asc)),
        Some(2) => items.sort_by(|a, b| compare_float(a.cost_price, b.cost_price, asc)),
        Some(3) => items.sort_by(|a, b| compare_float(a.selling_price, b.selling_price, asc)),
        Some(4) => items.sort_by(|a, b| compare_int(a.quantity_on_hand, b.quantity_on_hand, asc)),
        Some(5) => items.sort_by(|a, b| compare_float(a.margin_pct(), b.margin_pct(), asc)),
        _ => {}
    }
}

impl ProductsState {
    fn open_add(&mut self) {
        self.show_modal = true;
        self.editing_id = None;
        self.form_name.clear();
        self.form_barcode.clear();
        self.form_cost_price.clear();
        self.form_selling_price.clear();
        self.form_error = None;
    }

    fn open_edit(&mut self, id: i32, name: &str, barcode: &Option<String>, cost_price: f64, selling_price: f64) {
        self.show_modal = true;
        self.editing_id = Some(id);
        self.form_name = name.to_string();
        self.form_barcode = barcode.clone().unwrap_or_default();
        self.form_cost_price = format!("{:.0}", cost_price);
        self.form_selling_price = format!("{:.0}", selling_price);
        self.form_error = None;
    }

    fn close_modal(&mut self) {
        self.show_modal = false;
        self.editing_id = None;
    }

    fn save(&mut self, lang: Lang, conn: &mut diesel::SqliteConnection) -> bool {
        let name = self.form_name.trim().to_string();
        if name.is_empty() {
            self.form_error = Some(i18n::t("name_required", lang).to_string());
            return false;
        }
        let barcode = if self.form_barcode.trim().is_empty() { None } else { Some(self.form_barcode.trim().to_string()) };
        let cost_price = match self.form_cost_price.parse::<f64>() { Ok(v) if v >= 0.0 => v, _ => { self.form_error = Some(i18n::t("invalid_cost_price", lang).to_string()); return false } };
        let selling_price = match self.form_selling_price.parse::<f64>() { Ok(v) if v >= 0.0 => v, _ => { self.form_error = Some(i18n::t("invalid_selling_price", lang).to_string()); return false } };

        let new_product = monstock_core::models::NewProduct { name, barcode, cost_price, selling_price, quantity_on_hand: 0 };

        let result = if let Some(id) = self.editing_id {
            monstock_core::services::product_service::update(conn, id, &new_product).map(|_| ())
        } else {
            monstock_core::services::product_service::create(conn, &new_product).map(|_| ())
        };

        match result {
            Ok(()) => { self.close_modal(); true }
            Err(e) => { self.form_error = Some(e.to_string()); false }
        }
    }

    fn delete(&mut self, conn: &mut diesel::SqliteConnection, id: i32) {
        let _ = monstock_core::services::product_service::delete(conn, id);
    }
}

impl components::ModalScreen for ProductsState {
    fn save(&mut self, lang: Lang, conn: &mut diesel::SqliteConnection) -> bool {
        self.save(lang, conn)
    }
    fn close_modal(&mut self) {
        self.close_modal();
    }
}

pub fn show(ui: &mut egui::Ui, conn: &mut diesel::SqliteConnection, lang: Lang, is_dark: bool, state: &mut ProductsState) {
    page_header(ui, "02", i18n::t("products", lang), i18n::t("inventory", lang), is_dark);

    card(ui, is_dark, |ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(i18n::t("products", lang)).size(14.0).color(text_color(is_dark)).strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if primary_btn(ui, &format!("+ {}", i18n::t("add", lang))).clicked() {
                    state.open_add();
                }
            });
        });
    });

    ui.add_space(8.0);
    card(ui, is_dark, |ui| {
        state.pagination.total = monstock_core::services::product_service::count_all(conn).unwrap_or(0);
        let (mut products, error) = match monstock_core::services::product_service::find_paginated(conn, state.pagination.page, state.pagination.per_page) {
            Ok(list) => (list, None),
            Err(e) => (vec![], Some(format!("{}: {}", i18n::t("error", lang), e))),
        };
        let mut s = state.sort.clone();
        sort_products(&mut products, &s);
        components::data_table(ui, "products_grid", &[
            i18n::t("product", lang), i18n::t("barcode", lang), i18n::t("cost_price", lang),
            i18n::t("selling_price", lang), i18n::t("stock", lang), i18n::t("margin", lang), "",
        ], &products, error.as_deref(), Some(&mut s), |ui, p| {
            ui.vertical(|ui| {
                ui.add(egui::Label::new(
                    egui::RichText::new(&p.name).size(12.5).color(text_color(is_dark)).strong()
                ).extend());
                ui.label(egui::RichText::new(format!("PRD-{:03}", p.id)).size(11.0).monospace().color(TEXT_DIM));
            });
            mono_value(ui, p.barcode.as_deref().unwrap_or("-"), TEXT_SEC);
            mono_value(ui, &format!("{:.0}", p.cost_price), TEXT_SEC);
            mono_value(ui, &format!("{:.0}", p.selling_price), TEXT);
            ui.horizontal(|ui| {
                stock_bar(ui, p.quantity_on_hand, 50);
                ui.add_space(4.0);
                mono_value(ui, &format!("{}", p.quantity_on_hand), TEXT);
            });
            let margin = p.margin_pct();
            mono_value(ui, &format!("{:.0}%", margin), if margin >= 30.0 { GOOD } else if margin >= 10.0 { WARN } else { BAD });
            ui.horizontal(|ui| {
                if btn(ui, egui::RichText::new(i18n::t("edit", lang)).size(11.0)).clicked() {
                    state.open_edit(p.id, &p.name, &p.barcode, p.cost_price, p.selling_price);
                }
                if components::delete_btn(ui, is_dark).clicked() {
                    state.delete(conn, p.id);
                }
            });
        });
        state.sort = s;
        pagination_ui(ui, &mut state.pagination, is_dark);
    });

    if state.show_modal {
        let title = if state.editing_id.is_some() {
            format!("{} {}", i18n::t("edit", lang), i18n::t("products", lang))
        } else {
            format!("{} {}", i18n::t("add", lang), i18n::t("products", lang))
        };
        let err = state.form_error.clone();
        components::modal_window(ui.ctx(), &title, [400.0, 320.0], |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(i18n::t("name", lang)).size(13.0).color(text_color(is_dark)).strong());
                ui.add_space(8.0);
                ui.text_edit_singleline(&mut state.form_name);
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(i18n::t("barcode", lang)).size(13.0).color(text_color(is_dark)).strong());
                ui.add_space(8.0);
                ui.text_edit_singleline(&mut state.form_barcode);
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(i18n::t("cost_price", lang)).size(13.0).color(text_color(is_dark)).strong());
                ui.add_space(8.0);
                ui.add(egui::TextEdit::singleline(&mut state.form_cost_price).desired_width(80.0));
                ui.label(" DA");
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(i18n::t("selling_price", lang)).size(13.0).color(text_color(is_dark)).strong());
                ui.add_space(8.0);
                ui.add(egui::TextEdit::singleline(&mut state.form_selling_price).desired_width(80.0));
                ui.label(" DA");
            });
            components::modal_actions(ui, lang, err.as_deref(), conn, state);
        });
    }
}
