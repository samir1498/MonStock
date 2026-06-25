use egui;
use crate::i18n::{self, Lang};
use crate::style::*;

pub struct DashboardState;

impl Default for DashboardState {
    fn default() -> Self { Self }
}

fn stat_card(ui: &mut egui::Ui, label: &str, value: &str, val_color: egui::Color32, _is_dark: bool) {
    let frame = egui::Frame::new()
        .fill(SURFACE)
        .stroke(egui::Stroke::new(1.0, BORDER))
        .corner_radius(8)
        .inner_margin(egui::Margin::symmetric(18, 16));
    frame.show(ui, |ui| {
        ui.label(egui::RichText::new(label).size(11.0).color(TEXT_DIM).strong());
        ui.add_space(4.0);
        ui.label(egui::RichText::new(value).size(22.0).color(val_color).strong());
    });
}

pub fn show(ui: &mut egui::Ui, conn: &mut diesel::SqliteConnection, lang: Lang, is_dark: bool, _state: &mut DashboardState) {
    page_header(ui, "#", i18n::t("dashboard", lang), i18n::t("overview", lang), is_dark);

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let stats = monstock_core::services::dashboard_service::daily_stats(conn, &today).unwrap_or_default();
    let cost_total = monstock_core::services::sale_service::daily_cost_total(conn, &today).unwrap_or(0.0);
    let profit = monstock_core::services::dashboard_service::profit(stats.sales_total, cost_total, stats.expenses_total);
    let low_stock = monstock_core::services::dashboard_service::low_stock_products(conn, 5).unwrap_or_default();

    if !low_stock.is_empty() {
        let alert_frame = egui::Frame::new()
            .fill(RAISED)
            .stroke(egui::Stroke::new(1.0, BORDER))
            .corner_radius(8)
            .inner_margin(egui::Margin::symmetric(16, 12));
        alert_frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                egui::Frame::new()
                    .fill(SURFACE)
                    .stroke(egui::Stroke::new(1.0, BORDER))
                    .corner_radius(6)
                    .inner_margin(egui::Margin::symmetric(6, 4))
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("!").size(11.0).monospace().color(WARN));
                    });
                ui.add_space(8.0);
                ui.label(egui::RichText::new(
                    format!("{} {} {}", low_stock.len(), i18n::t("items", lang), i18n::t("low_stock", lang))
                ).size(12.5).strong());
            });
        });
        ui.add_space(12.0);
    }

    ui.label(egui::RichText::new(i18n::t("stats_today", lang)).size(14.0).strong());
    ui.add_space(8.0);

    card(ui, is_dark, |ui| {
        egui::Grid::new("stats_grid")
            .min_col_width(160.0)
            .max_col_width(200.0)
            .show(ui, |ui| {
                stat_card(ui, i18n::t("sales", lang), &format!("{:.0} DA", stats.sales_total), GOOD, is_dark);
                stat_card(ui, i18n::t("transactions", lang), &format!("{}", stats.transaction_count), ACCENT, is_dark);
                ui.end_row();
                stat_card(ui, i18n::t("expenses", lang), &format!("{:.0} DA", stats.expenses_total), BAD, is_dark);
                stat_card(ui, i18n::t("profit", lang), &format!("{:.0} DA", profit), if profit >= 0.0 { GOOD } else { BAD }, is_dark);
                ui.end_row();
            });
    });

    ui.add_space(20.0);
    ui.label(egui::RichText::new(i18n::t("low_stock", lang)).size(14.0).strong());
    ui.add_space(8.0);

    card(ui, is_dark, |ui| {
        if low_stock.is_empty() {
            ui.colored_label(TEXT_DIM, i18n::t("all_stocked", lang));
        } else {
            egui::Grid::new("low_stock_grid")
                .striped(true)
                .min_col_width(100.0)
                .show(ui, |ui| {
                    ui.strong(i18n::t("name", lang));
                    ui.strong(i18n::t("stock", lang));
                    ui.strong(i18n::t("price", lang));
                    ui.end_row();

                    for p in &low_stock {
                        ui.label(egui::RichText::new(&p.name).size(13.0));
                        ui.colored_label(BAD, format!("{}", p.quantity_on_hand));
                        ui.label(egui::RichText::new(format!("{:.0} DA", p.selling_price)).size(13.0));
                        ui.end_row();
                    }
                });
        }
    });
}
