use egui;
use crate::i18n::{self, Lang};
use crate::style::*;
use crate::components;

pub struct ExpensesState {
    pub show_modal: bool,
    pub filter_start: String,
    pub filter_end: String,
    pub form_date: String,
    pub form_category: String,
    pub form_description: String,
    pub form_amount: String,
    pub form_error: Option<String>,
    pub pagination: PaginationState,
    pub sort: SortState,
}

fn sort_expenses(items: &mut [monstock_core::models::Expense], sort: &SortState) {
    let asc = sort.ascending;
    match sort.column {
        Some(0) => items.sort_by(|a, b| compare_str(&a.date, &b.date, asc)),
        Some(1) => items.sort_by(|a, b| compare_str(&a.category, &b.category, asc)),
        Some(2) => items.sort_by(|a, b| compare_str(a.description.as_deref().unwrap_or(""), b.description.as_deref().unwrap_or(""), asc)),
        Some(3) => items.sort_by(|a, b| compare_float(a.amount, b.amount, asc)),
        _ => {}
    }
}

impl Default for ExpensesState {
    fn default() -> Self {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        Self {
            show_modal: false,
            filter_start: chrono::Local::now().format("%Y-%m-01").to_string(),
            filter_end: today.clone(),
            form_date: today,
            form_category: "Livraison".to_string(),
            form_description: String::new(),
            form_amount: String::new(),
            form_error: None,
            pagination: Default::default(),
            sort: Default::default(),
        }
    }
}

impl components::ModalScreen for ExpensesState {
    fn save(&mut self, lang: Lang, conn: &mut diesel::SqliteConnection) -> bool {
        self.save(lang, conn)
    }
    fn close_modal(&mut self) {
        self.close_modal();
    }
}

impl ExpensesState {
    fn open_add(&mut self) {
        self.show_modal = true;
        self.form_description.clear();
        self.form_amount.clear();
        self.form_error = None;
    }

    fn close_modal(&mut self) { self.show_modal = false; }

    fn save(&mut self, lang: Lang, conn: &mut diesel::SqliteConnection) -> bool {
        let amount: f64 = match self.form_amount.parse() { Ok(v) if v >= 0.0 => v, _ => { self.form_error = Some(i18n::t("invalid_amount", lang).to_string()); return false } };
        let expense = monstock_core::models::NewExpense {
            date: self.form_date.clone(),
            category: self.form_category.clone(),
            description: if self.form_description.trim().is_empty() { None } else { Some(self.form_description.trim().to_string()) },
            amount,
        };
        match monstock_core::services::expense_service::create(conn, &expense) {
            Ok(_) => { self.close_modal(); true }
            Err(e) => { self.form_error = Some(e.to_string()); false }
        }
    }

    fn delete(&mut self, conn: &mut diesel::SqliteConnection, expense_id: i32) {
        let _ = monstock_core::services::expense_service::delete(conn, expense_id);
    }
}

pub fn show(ui: &mut egui::Ui, conn: &mut diesel::SqliteConnection, lang: Lang, is_dark: bool, state: &mut ExpensesState) {
    page_header(ui, "05", i18n::t("expenses", lang), i18n::t("overview", lang), is_dark);

    card(ui, is_dark, |ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(i18n::t("date", lang)).size(13.0).color(text_color(is_dark)).strong());
            ui.add_space(4.0);
            ui.add(egui::TextEdit::singleline(&mut state.filter_start).desired_width(100.0));
            ui.add_space(4.0);
            ui.label(egui::RichText::new(">").size(14.0).color(TEXT_DIM));
            ui.add_space(4.0);
            ui.add(egui::TextEdit::singleline(&mut state.filter_end).desired_width(100.0));
            ui.add_space(12.0);
            let total = monstock_core::services::expense_service::total_by_range(conn, &state.filter_start, &state.filter_end).unwrap_or(0.0);
            amount_text(ui, &format!("{:.0} DA", total), BAD);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if primary_btn(ui, &format!("+ {}", i18n::t("add", lang))).clicked() { state.open_add(); }
            });
        });
    });

    ui.add_space(8.0);
    card(ui, is_dark, |ui| {
        state.pagination.total = monstock_core::services::expense_service::count_by_date_range(conn, &state.filter_start, &state.filter_end).unwrap_or(0);
        let (mut expenses, error) = match monstock_core::services::expense_service::find_paginated(conn, &state.filter_start, &state.filter_end, state.pagination.page, state.pagination.per_page) {
            Ok(list) => (list, None),
            Err(e) => (vec![], Some(format!("{}: {}", i18n::t("error", lang), e))),
        };
        let mut s = state.sort.clone();
        sort_expenses(&mut expenses, &s);
        components::data_table(ui, "expenses_grid", &[
            i18n::t("date", lang), i18n::t("category", lang), i18n::t("notes", lang),
            i18n::t("amount", lang), "",
        ], &expenses, error.as_deref(), Some(&mut s), |ui, e| {
            mono_value(ui, &e.date, TEXT_SEC);
            tag(ui, &e.category, ACCENT, is_dark);
            ui.label(egui::RichText::new(e.description.as_deref().unwrap_or("-")).size(13.0).color(TEXT_SEC));
            amount_text(ui, &format!("{:.0} DA", e.amount), BAD);
            if components::delete_btn(ui, is_dark).clicked() { state.delete(conn, e.id); }
        });
        state.sort = s;
        pagination_ui(ui, &mut state.pagination, is_dark);
    });

    if state.show_modal {
        let title = format!("{} {}", i18n::t("add", lang), i18n::t("expenses", lang));
        let err = state.form_error.clone();
        components::modal_window(ui.ctx(), &title, [400.0, 300.0], |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(i18n::t("date", lang)).size(13.0).color(text_color(is_dark)).strong());
                ui.add_space(8.0);
                ui.add(egui::TextEdit::singleline(&mut state.form_date).desired_width(120.0));
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(i18n::t("category", lang)).size(13.0).color(text_color(is_dark)).strong());
                ui.add_space(8.0);
                let categories = monstock_core::services::expense_category_service::find_all(conn).unwrap_or_default();
                egui::ComboBox::from_id_salt("expense_category").selected_text(&state.form_category).width(200.0).show_ui(ui, |ui| {
                    for c in &categories { ui.selectable_value(&mut state.form_category, c.name.clone(), &c.name); }
                });
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(i18n::t("notes", lang)).size(13.0).color(text_color(is_dark)).strong());
                ui.add_space(8.0);
                ui.add(egui::TextEdit::singleline(&mut state.form_description).desired_width(200.0).hint_text(i18n::t("notes", lang)));
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(i18n::t("amount", lang)).size(13.0).color(text_color(is_dark)).strong());
                ui.add_space(8.0);
                ui.add(egui::TextEdit::singleline(&mut state.form_amount).desired_width(100.0));
                ui.label(" DA");
            });
            components::modal_actions(ui, lang, err.as_deref(), conn, state);
        });
    }
}
