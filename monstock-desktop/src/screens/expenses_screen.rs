use egui;
use crate::i18n::{self, Lang};
use crate::style::*;

pub struct ExpensesState {
    pub show_modal: bool,
    pub filter_start: String,
    pub filter_end: String,
    pub form_date: String,
    pub form_category: String,
    pub form_description: String,
    pub form_amount: String,
    pub form_error: Option<String>,
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
        }
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

    fn save(&mut self, conn: &mut diesel::SqliteConnection) -> bool {
        let amount: f64 = match self.form_amount.parse() { Ok(v) if v >= 0.0 => v, _ => { self.form_error = Some("Invalid amount".to_string()); return false } };
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
            ui.label(egui::RichText::new(i18n::t("date", lang)).size(13.0).strong());
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
        let expenses = monstock_core::services::expense_service::find_by_date_range(conn, &state.filter_start, &state.filter_end);
        match expenses {
            Ok(list) => {
                egui::Grid::new("expenses_grid").striped(true).min_col_width(80.0).show(ui, |ui| {
                    table_header(ui, i18n::t("date", lang));
                    table_header(ui, "Categorie");
                    table_header(ui, i18n::t("notes", lang));
                    table_header(ui, "Montant");
                    table_header(ui, "");
                    ui.end_row();

                    for e in &list {
                        mono_value(ui, &e.date, TEXT_SEC);
                        tag(ui, &e.category, ACCENT);
                        ui.label(egui::RichText::new(e.description.as_deref().unwrap_or("-")).size(13.0).color(TEXT_SEC));
                        amount_text(ui, &format!("{:.0} DA", e.amount), BAD);
                        let d = btn_custom(ui, egui::Button::new(egui::RichText::new("X").size(10.0).color(TEXT_DIM)).fill(egui::Color32::TRANSPARENT).min_size(egui::vec2(20.0, 20.0)));
                        if d.clicked() { state.delete(conn, e.id); }
                        ui.end_row();
                    }
                });
            }
            Err(e) => { ui.colored_label(BAD, format!("{}: {}", i18n::t("error", lang), e)); }
        }
    });

    if state.show_modal {
        egui::Window::new(format!("{} {}", i18n::t("add", lang), i18n::t("expenses", lang)))
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .fixed_size([400.0, 300.0]).collapsible(false).title_bar(true).resizable(false).movable(false)
            .show(ui.ctx(), |ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(i18n::t("date", lang)).size(13.0).strong());
                    ui.add_space(8.0);
                    ui.add(egui::TextEdit::singleline(&mut state.form_date).desired_width(120.0));
                });
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Categorie").size(13.0).strong());
                    ui.add_space(8.0);
                    let categories = monstock_core::repos::expense_category_repo::find_all_categories(conn).unwrap_or_default();
                    egui::ComboBox::from_id_salt("expense_category").selected_text(&state.form_category).width(200.0).show_ui(ui, |ui| {
                        for c in &categories { ui.selectable_value(&mut state.form_category, c.name.clone(), &c.name); }
                    });
                });
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(i18n::t("notes", lang)).size(13.0).strong());
                    ui.add_space(8.0);
                    ui.add(egui::TextEdit::singleline(&mut state.form_description).desired_width(200.0).hint_text(i18n::t("notes", lang)));
                });
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Montant").size(13.0).strong());
                    ui.add_space(8.0);
                    ui.add(egui::TextEdit::singleline(&mut state.form_amount).desired_width(100.0));
                    ui.label(" DA");
                });
                ui.add_space(12.0);
                if let Some(ref err) = state.form_error { ui.colored_label(BAD, err); ui.add_space(4.0); }
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if btn(ui, i18n::t("save", lang)).clicked() { state.save(conn); }
                        ui.add_space(8.0);
                        if btn(ui, i18n::t("cancel", lang)).clicked() { state.close_modal(); }
                    });
                });
            });
    }
}
