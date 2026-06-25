use egui;
use crate::i18n::{self, Lang};
use crate::style::*;

pub struct SalesState {
    pub show_sale_modal: bool,
    pub filter_date: String,
    pub form_items: Vec<SaleItemForm>,
    pub form_error: Option<String>,
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
        }
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

    fn save(&mut self, conn: &mut diesel::SqliteConnection) -> bool {
        if self.form_items.is_empty() { self.form_error = Some("At least one item is required".to_string()); return false; }

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
        let transactions = monstock_core::services::sale_service::find_transactions_by_date(conn, &state.filter_date);
        match transactions {
            Ok(list) => {
                egui::Grid::new("sales_grid").striped(true).min_col_width(80.0).show(ui, |ui| {
                    table_header(ui, "ID");
                    table_header(ui, i18n::t("date", lang));
                    table_header(ui, i18n::t("total", lang));
                    ui.end_row();

                    for tx in &list {
                        mono_value(ui, &format!("{}", tx.id), TEXT);
                        mono_value(ui, &tx.timestamp, TEXT_SEC);
                        amount_text(ui, &format!("{:.0} DA", tx.total), TEXT);
                        ui.end_row();
                    }
                });
            }
            Err(e) => { ui.colored_label(BAD, format!("{}: {}", i18n::t("error", lang), e)); }
        }
    });

    if state.show_sale_modal {
        egui::Window::new(format!("{} {}", i18n::t("add", lang), i18n::t("sales", lang)))
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .fixed_size([500.0, 450.0]).collapsible(false).title_bar(true).resizable(false).movable(false)
            .show(ui.ctx(), |ui| {
                ui.add_space(8.0);
                ui.label(egui::RichText::new("Produits disponibles").size(14.0).color(text_color(is_dark)).strong());
                ui.add_space(4.0);

                let products = monstock_core::services::product_service::find_all(conn).unwrap_or_default();
                egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                    egui::Grid::new("product_picker_grid").striped(true).min_col_width(60.0).show(ui, |ui| {
                        table_header(ui, "Produit");
                        table_header(ui, i18n::t("stock", lang));
                        table_header(ui, "Prix");
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
                ui.label(egui::RichText::new("Articles saisis").size(14.0).color(text_color(is_dark)).strong());

                egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                    egui::Grid::new("sale_items_grid").striped(true).min_col_width(60.0).show(ui, |ui| {
                        table_header(ui, "Produit");
                        table_header(ui, "Qte");
                        table_header(ui, "Prix unit.");
                        table_header(ui, "");
                        ui.end_row();

                        let mut remove_idx: Option<usize> = None;
                        for (idx, item) in state.form_items.iter_mut().enumerate() {
                            ui.label(egui::RichText::new(&item.product_name).size(13.0).color(text_color(is_dark)));
                            ui.add(egui::TextEdit::singleline(&mut item.quantity).desired_width(50.0));
                            ui.add(egui::TextEdit::singleline(&mut item.selling_price).desired_width(60.0));
                            let r = btn_custom(ui, egui::Button::new(egui::RichText::new("X").size(12.0).color(TEXT_DIM)).fill(egui::Color32::TRANSPARENT).min_size(egui::vec2(20.0, 20.0)));
                            if r.clicked() { remove_idx = Some(idx); }
                            ui.end_row();
                        }
                        if let Some(idx) = remove_idx { state.remove_item(idx); }
                    });
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
