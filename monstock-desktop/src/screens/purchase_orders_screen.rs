use egui;
use crate::i18n::{self, Lang};
use crate::style::*;
use crate::components;

#[derive(Clone)]
pub(crate) struct PoItemForm {
    product_name: String,
    quantity: String,
    unit_cost: String,
}

pub struct PurchaseOrdersState {
    pub show_modal: bool,
    pub editing_id: Option<i32>,
    pub form_supplier_id: Option<i32>,
    pub form_notes: String,
    pub form_items: Vec<PoItemForm>,
    pub form_error: Option<String>,
    pub pagination: PaginationState,
}

impl Default for PurchaseOrdersState {
    fn default() -> Self {
        Self {
            show_modal: false,
            editing_id: None,
            form_supplier_id: None,
            form_notes: String::new(),
            form_items: vec![PoItemForm { product_name: String::new(), quantity: "1".to_string(), unit_cost: "0".to_string() }],
            form_error: None,
            pagination: Default::default(),
        }
    }
}

impl PurchaseOrdersState {
    fn open_add(&mut self) {
        self.show_modal = true;
        self.editing_id = None;
        self.form_supplier_id = None;
        self.form_notes.clear();
        self.form_items = vec![PoItemForm { product_name: String::new(), quantity: "1".to_string(), unit_cost: "0".to_string() }];
        self.form_error = None;
    }

    fn close_modal(&mut self) {
        self.show_modal = false;
        self.editing_id = None;
    }

    fn generate_po_number() -> String {
        format!("PO-{}", chrono::Local::now().format("%Y%m%d-%H%M%S"))
    }

    fn save(&mut self, conn: &mut diesel::SqliteConnection) -> bool {
        if self.form_items.is_empty() || self.form_items.iter().all(|i| i.product_name.trim().is_empty()) {
            self.form_error = Some("At least one item is required".to_string());
            return false;
        }

        let mut items = Vec::new();
        for item in &self.form_items {
            let name = item.product_name.trim().to_string();
            if name.is_empty() { continue; }
            let quantity: i32 = match item.quantity.parse() { Ok(v) if v > 0 => v, _ => { self.form_error = Some(format!("Invalid quantity for '{}'", name)); return false } };
            let unit_cost: f64 = match item.unit_cost.parse() { Ok(v) if v >= 0.0 => v, _ => { self.form_error = Some(format!("Invalid unit cost for '{}'", name)); return false } };
            items.push(monstock_core::services::purchase_order_service::PurchaseOrderItemInput { product_name: name, quantity, unit_cost });
        }

        let input = monstock_core::services::purchase_order_service::PurchaseOrderInput {
            purchase_order_number: Self::generate_po_number(),
            supplier_id: self.form_supplier_id,
            notes: if self.form_notes.trim().is_empty() { None } else { Some(self.form_notes.trim().to_string()) },
        };

        match monstock_core::services::purchase_order_service::create_purchase_order(conn, &input, &items) {
            Ok(_) => { self.close_modal(); true }
            Err(e) => { self.form_error = Some(e.to_string()); false }
        }
    }

    fn receive(&mut self, conn: &mut diesel::SqliteConnection, po_id: i32) {
        let _ = monstock_core::services::purchase_order_service::receive_purchase_order(conn, po_id);
    }

    fn delete(&mut self, conn: &mut diesel::SqliteConnection, po_id: i32) {
        let _ = monstock_core::services::purchase_order_service::delete(conn, po_id);
    }

    fn add_item_row(&mut self) {
        self.form_items.push(PoItemForm { product_name: String::new(), quantity: "1".to_string(), unit_cost: "0".to_string() });
    }

    fn remove_item_row(&mut self, index: usize) {
        self.form_items.remove(index);
        if self.form_items.is_empty() { self.add_item_row(); }
    }
}

pub fn show(ui: &mut egui::Ui, conn: &mut diesel::SqliteConnection, lang: Lang, is_dark: bool, state: &mut PurchaseOrdersState) {
    let subtitle = format!("{} > {}", i18n::t("draft", lang), i18n::t("received", lang));
    page_header(ui, "03", i18n::t("purchase_orders", lang), &subtitle, is_dark);

    card(ui, is_dark, |ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(i18n::t("purchase_orders", lang)).size(14.0).color(text_color(is_dark)).strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if primary_btn(ui, &format!("+ {}", i18n::t("add", lang))).clicked() {
                    state.open_add();
                }
            });
        });
    });

    ui.add_space(8.0);
    card(ui, is_dark, |ui| {
        state.pagination.total = monstock_core::services::purchase_order_service::count_all(conn).unwrap_or(0);
        let (orders, error) = match monstock_core::services::purchase_order_service::find_paginated(conn, state.pagination.page, state.pagination.per_page) {
            Ok(list) => (list, None),
            Err(e) => (vec![], Some(format!("{}: {}", i18n::t("error", lang), e))),
        };
        components::data_table(ui, "po_grid", &[
            i18n::t("name", lang), i18n::t("supplier", lang), i18n::t("total", lang),
            i18n::t("status", lang), i18n::t("date", lang), "", "",
        ], &orders, error.as_deref(), |ui, po| {
            mono_value(ui, &po.purchase_order_number, TEXT);
            let supplier_name = monstock_core::repos::purchase_order_repo::purchase_order_supplier_name(conn, po.supplier_id);
            mono_value(ui, supplier_name.as_deref().unwrap_or("-"), TEXT_SEC);
            amount_text(ui, &format!("{:.0}", po.total), TEXT);
            let (sc, sl) = if po.status == "Received" { (GOOD, i18n::t("received", lang)) } else { (WARN, i18n::t("draft", lang)) };
            tag(ui, sl, sc, is_dark);
            ui.label(egui::RichText::new(&po.created_at).size(12.0).color(TEXT_SEC));
            if po.status == "Draft" {
                let r = btn_custom(ui,
                    egui::Button::new(egui::RichText::new(i18n::t("received", lang)).size(11.0).color(TEXT))
                        .fill(GOOD).corner_radius(4).min_size(egui::vec2(60.0, 22.0))
                );
                if r.clicked() { state.receive(conn, po.id); }
            } else {
                ui.label("");
            }
            if components::delete_btn(ui, is_dark).clicked() { state.delete(conn, po.id); }
        });
        pagination_ui(ui, &mut state.pagination, is_dark);
    });

    if state.show_modal {
        let title = format!("{} {}", i18n::t("add", lang), i18n::t("purchase_orders", lang));
        let err = state.form_error.clone();
        components::modal_window(ui.ctx(), &title, [480.0, 400.0], |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(i18n::t("supplier", lang)).size(13.0).color(text_color(is_dark)).strong());
                ui.add_space(8.0);
                let suppliers = monstock_core::repos::supplier_repo::find_all_suppliers(conn).unwrap_or_default();
                let cur = state.form_supplier_id.and_then(|id| suppliers.iter().find(|s| s.id == id)).map(|s| s.name.clone()).unwrap_or_else(|| "-".to_string());
                egui::ComboBox::from_id_salt("supplier_combo").selected_text(&cur).width(200.0).show_ui(ui, |ui| {
                    ui.selectable_value(&mut state.form_supplier_id, None, "-");
                    for s in &suppliers { let id = Some(s.id); ui.selectable_value(&mut state.form_supplier_id, id, &s.name); }
                });
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(i18n::t("notes", lang)).size(13.0).color(text_color(is_dark)).strong());
                ui.add_space(8.0);
                ui.add(egui::TextEdit::singleline(&mut state.form_notes).desired_width(250.0).hint_text(i18n::t("notes", lang)));
            });
            ui.add_space(12.0);
            ui.separator();
            ui.add_space(4.0);
            ui.label(egui::RichText::new(i18n::t("entered_items", lang)).size(14.0).color(text_color(is_dark)).strong());

            egui::ScrollArea::vertical().max_height(160.0).show(ui, |ui| {
                egui::Grid::new("po_items_grid").striped(true).min_col_width(60.0).show(ui, |ui| {
                    table_header(ui, i18n::t("product", lang));
                    table_header(ui, i18n::t("quantity", lang));
                    table_header(ui, i18n::t("unit_price", lang));
                    table_header(ui, "");
                    ui.end_row();

                    let mut remove_idx: Option<usize> = None;
                    for (idx, item) in state.form_items.iter_mut().enumerate() {
                        ui.add(egui::TextEdit::singleline(&mut item.product_name).desired_width(140.0).hint_text(i18n::t("name", lang)));
                        ui.add(egui::TextEdit::singleline(&mut item.quantity).desired_width(50.0));
                        ui.add(egui::TextEdit::singleline(&mut item.unit_cost).desired_width(60.0));
                        if components::delete_btn(ui, is_dark).clicked() { remove_idx = Some(idx); }
                        ui.end_row();
                    }
                    if let Some(idx) = remove_idx { state.remove_item_row(idx); }
                });
            });

            ui.add_space(4.0);
            if btn(ui, egui::RichText::new(format!("+ {}", i18n::t("add", lang))).size(12.0).color(ACCENT)).clicked() { state.add_item_row(); }
            components::modal_actions(ui, lang, err.as_deref(), |is_save| {
                if is_save { state.save(conn); }
                else { state.close_modal(); }
            });
        });
    }
}
