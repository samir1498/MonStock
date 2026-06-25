use egui;
use crate::i18n::{self, Lang};
use crate::style::*;

pub struct ProductsState {
    pub show_modal: bool,
    pub editing_id: Option<i32>,
    pub form_name: String,
    pub form_barcode: String,
    pub form_cost_price: String,
    pub form_selling_price: String,
    pub form_error: Option<String>,
}

impl Default for ProductsState {
    fn default() -> Self {
        Self {
            show_modal: false,
            editing_id: None,
            form_name: String::new(),
            form_barcode: String::new(),
            form_cost_price: String::new(),
            form_selling_price: String::new(),
            form_error: None,
        }
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

    fn save(&mut self, conn: &mut diesel::SqliteConnection) -> bool {
        let name = self.form_name.trim().to_string();
        if name.is_empty() {
            self.form_error = Some("Name is required".to_string());
            return false;
        }
        let barcode = if self.form_barcode.trim().is_empty() { None } else { Some(self.form_barcode.trim().to_string()) };
        let cost_price = match self.form_cost_price.parse::<f64>() { Ok(v) if v >= 0.0 => v, _ => { self.form_error = Some("Invalid cost price".to_string()); return false } };
        let selling_price = match self.form_selling_price.parse::<f64>() { Ok(v) if v >= 0.0 => v, _ => { self.form_error = Some("Invalid selling price".to_string()); return false } };

        let new_product = monstock_core::models::NewProduct { name, barcode, cost_price, selling_price };

        let result = if let Some(id) = self.editing_id {
            monstock_core::services::product_service::update(conn, id, &new_product).and_then(|_| Ok(()))
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

fn initials(name: &str) -> String {
    name.split_whitespace().filter_map(|w| w.chars().next()).take(2).collect()
}

pub fn show(ui: &mut egui::Ui, conn: &mut diesel::SqliteConnection, lang: Lang, is_dark: bool, state: &mut ProductsState) {
    page_header(ui, "02", i18n::t("products", lang), i18n::t("inventory", lang), is_dark);

    card(ui, is_dark, |ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(i18n::t("products", lang)).size(14.0).strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if primary_btn(ui, &format!("+ {}", i18n::t("add", lang))).clicked() {
                    state.open_add();
                }
            });
        });
    });

    ui.add_space(8.0);
    card(ui, is_dark, |ui| {
        let products = monstock_core::services::product_service::find_all(conn);
        match products {
            Ok(list) => {
                egui::Grid::new("products_grid")
                    .striped(true)
                    .min_col_width(80.0)
                    .show(ui, |ui| {
                        table_header(ui, "");
                        table_header(ui, i18n::t("name", lang));
                        table_header(ui, i18n::t("barcode", lang));
                        table_header(ui, i18n::t("cost_price", lang));
                        table_header(ui, i18n::t("selling_price", lang));
                        table_header(ui, i18n::t("stock", lang));
                        table_header(ui, i18n::t("margin", lang));
                        table_header(ui, "");
                        ui.end_row();

                        for p in &list {
                            let _row_id = ui.next_auto_id();
                            let _response = egui::Frame::new()
                                .inner_margin(egui::Margin::symmetric(4, 4))
                                .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    product_avatar(ui, &initials(&p.name));
                                    ui.add_space(4.0);
                                    ui.vertical(|ui| {
                                        ui.label(egui::RichText::new(&p.name).size(12.5).strong());
                                        ui.label(egui::RichText::new(format!("PRD-{:03}", p.id)).size(11.0).monospace().color(TEXT_DIM));
                                    });
                                });
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
                                if btn(ui, egui::RichText::new("X").size(11.0).color(BAD)).clicked() {
                                    state.delete(conn, p.id);
                                }
                            });
                            ui.end_row();
                        }
                    });
            }
            Err(e) => { ui.colored_label(BAD, format!("{}: {}", i18n::t("error", lang), e)); }
        }
    });

    if state.show_modal {
        let title = if state.editing_id.is_some() {
            format!("{} {}", i18n::t("edit", lang), i18n::t("products", lang))
        } else {
            format!("{} {}", i18n::t("add", lang), i18n::t("products", lang))
        };

        egui::Window::new(&title)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .fixed_size([400.0, 320.0])
            .collapsible(false)
            .title_bar(true)
            .resizable(false)
            .movable(false)
            .show(ui.ctx(), |ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(i18n::t("name", lang)).size(13.0).strong());
                    ui.add_space(8.0);
                    ui.text_edit_singleline(&mut state.form_name);
                });
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(i18n::t("barcode", lang)).size(13.0).strong());
                    ui.add_space(8.0);
                    ui.text_edit_singleline(&mut state.form_barcode);
                });
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(i18n::t("cost_price", lang)).size(13.0).strong());
                    ui.add_space(8.0);
                    ui.add(egui::TextEdit::singleline(&mut state.form_cost_price).desired_width(80.0));
                    ui.label(" DA");
                });
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(i18n::t("selling_price", lang)).size(13.0).strong());
                    ui.add_space(8.0);
                    ui.add(egui::TextEdit::singleline(&mut state.form_selling_price).desired_width(80.0));
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
