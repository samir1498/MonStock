#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod i18n;
mod screens;
mod style;

use i18n::{t, Lang};
use screens::*;
use style::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Screen {
    Dashboard,
    Products,
    PurchaseOrders,
    Sales,
    Expenses,
}

const NAV_ITEMS: &[(Screen, &str, &str)] = &[
    (Screen::Dashboard, "dashboard", "01"),
    (Screen::Products, "products", "02"),
    (Screen::PurchaseOrders, "purchase_orders", "03"),
    (Screen::Sales, "sales", "04"),
    (Screen::Expenses, "expenses", "05"),
];

struct MonStockApp {
    conn: diesel::SqliteConnection,
    screen: Screen,
    lang: Lang,
    products_state: screens::products_screen::ProductsState,
    purchase_orders_state: screens::purchase_orders_screen::PurchaseOrdersState,
    sales_state: screens::sales_screen::SalesState,
    expenses_state: screens::expenses_screen::ExpensesState,
    dashboard_state: screens::dashboard_screen::DashboardState,
}

impl MonStockApp {
    fn new(conn: diesel::SqliteConnection) -> Self {
        Self {
            conn,
            screen: Screen::Dashboard,
            lang: Lang::Fr,
            products_state: Default::default(),
            purchase_orders_state: Default::default(),
            sales_state: Default::default(),
            expenses_state: Default::default(),
            dashboard_state: Default::default(),
        }
    }
}

impl eframe::App for MonStockApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let is_dark = ui.visuals().dark_mode;
        let sidebar_bg = if is_dark { SURFACE } else { egui::Color32::from_rgb(245, 245, 250) };
        let main_bg = if is_dark { BG } else { egui::Color32::from_rgb(250, 250, 252) };
        let nav_text = if is_dark { TEXT_SEC } else { egui::Color32::from_rgb(100, 100, 110) };
        let nav_text_active = if is_dark { TEXT } else { egui::Color32::from_rgb(20, 20, 28) };
        let active_bg = if is_dark { RAISED } else { egui::Color32::from_rgb(255, 255, 255) };

        egui::Panel::left("sidebar")
            .default_size(220.0)
            .resizable(false)
            .frame(egui::Frame::new().fill(sidebar_bg))
            .show_inside(ui, |ui| {
                ui.add_space(28.0);
                ui.horizontal(|ui| {
                    ui.add_space(24.0);
                    egui::Frame::new()
                        .fill(RAISED)
                        .stroke(egui::Stroke::new(1.5, BORDER_STRONG))
                        .corner_radius(6)
                        .inner_margin(egui::Margin::symmetric(6, 4))
                        .show(ui, |ui| {
                            ui.allocate_ui(egui::vec2(10.0, 10.0), |ui| {
                                ui.painter().rect_filled(ui.available_rect_before_wrap(),
                                    egui::CornerRadius::same(2),
                                    if is_dark { RAISED } else { egui::Color32::from_rgb(240, 240, 245) });
                            });
                        });
                    ui.add_space(6.0);
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new(t("monstock", self.lang))
                            .size(17.0).strong());
                        ui.add_space(1.0);
                        ui.label(egui::RichText::new(t("app_subtitle", self.lang))
                            .size(12.0).color(TEXT_DIM));
                    });
                });

                ui.add_space(12.0);

                for (screen, key, num) in NAV_ITEMS {
                    let selected = *screen == self.screen;
                    let text_c = if selected { nav_text_active } else { nav_text };
                    let bg_c = if selected { active_bg } else { egui::Color32::TRANSPARENT };
                    let border_c = if selected { BORDER_STRONG } else { egui::Color32::TRANSPARENT };

                    let frame_res = egui::Frame::new()
                        .fill(bg_c)
                        .stroke(egui::Stroke::new(1.0, border_c))
                        .corner_radius(6)
                        .inner_margin(egui::Margin::symmetric(12, 7))
                        .show(ui, |ui| {
                            ui.set_min_width(ui.available_width());
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(*num).size(12.0)
                                    .monospace().color(TEXT_DIM));
                                ui.add_space(8.0);
                                ui.label(egui::RichText::new(t(key, self.lang))
                                    .size(14.0).color(text_c));
                            });
                        });

                    if selected {
                        let rect = frame_res.response.rect;
                        let indicator = egui::Rect::from_min_max(
                            egui::pos2(rect.left() + 4.0, rect.top() + (rect.height() - 14.0) / 2.0),
                            egui::pos2(rect.left() + 7.0, rect.top() + (rect.height() + 14.0) / 2.0)
                        );
                        ui.painter().rect_filled(indicator, egui::CornerRadius::same(1), TEXT);
                    }

                    let id = ui.make_persistent_id(key);
                    let sense = ui.interact(frame_res.response.rect, id, egui::Sense::click());
                    if sense.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }
                    if sense.clicked() {
                        self.screen = *screen;
                    }
                }

                ui.add_space(14.0);
                ui.separator();
                ui.add_space(8.0);

                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 2.0;

                    ui.horizontal(|ui| {
                        ui.add_space(12.0); // Align with nav items text start
                        ui.label(egui::RichText::new("System").size(10.0)
                            .color(TEXT_DIM).strong());
                    });
                    ui.add_space(4.0);

                    let theme_text = if is_dark { t("light_mode", self.lang) } else { t("dark_mode", self.lang) };
                    if system_btn(ui, SystemIcon::SunMoon, &theme_text, nav_text, is_dark, sidebar_bg).clicked() {
                        let mut visuals = ui.ctx().global_style().visuals.clone();
                        visuals.dark_mode = !is_dark;
                        ui.ctx().set_visuals(visuals);
                    }

                    let lang_label = self.lang.label();
                    let lang_text = format!("{}: {}", t("language", self.lang), lang_label);
                    if system_btn(ui, SystemIcon::Globe, &lang_text, nav_text, is_dark, sidebar_bg).clicked() {
                        self.lang = self.lang.toggle();
                    }
                });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(main_bg).inner_margin(egui::Margin::same(28)))
            .show_inside(ui, |ui| {
                match self.screen {
                    Screen::Dashboard => {
                        if let Some(next_screen) = dashboard_screen::show(ui, &mut self.conn, self.lang,
                            is_dark, &mut self.dashboard_state) {
                            self.screen = next_screen;
                        }
                    }
                    Screen::Products => products_screen::show(ui, &mut self.conn, self.lang,
                        is_dark, &mut self.products_state),
                    Screen::PurchaseOrders => purchase_orders_screen::show(ui, &mut self.conn,
                        self.lang, is_dark, &mut self.purchase_orders_state),
                    Screen::Sales => sales_screen::show(ui, &mut self.conn, self.lang,
                        is_dark, &mut self.sales_state),
                    Screen::Expenses => expenses_screen::show(ui, &mut self.conn, self.lang,
                        is_dark, &mut self.expenses_state),
                }
            });
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum SystemIcon {
    SunMoon,
    Globe,
}

fn draw_sun(painter: &egui::Painter, center: egui::Pos2, color: egui::Color32) {
    painter.circle_filled(center, 3.5, color);
    for i in 0..8 {
        let angle = (i as f32) * std::f32::consts::TAU / 8.0;
        let start = center + egui::vec2(angle.cos() * 5.0, angle.sin() * 5.0);
        let end = center + egui::vec2(angle.cos() * 7.5, angle.sin() * 7.5);
        painter.line_segment([start, end], egui::Stroke::new(1.0, color));
    }
}

fn draw_moon(painter: &egui::Painter, center: egui::Pos2, color: egui::Color32, bg_color: egui::Color32) {
    painter.circle_filled(center, 5.0, color);
    painter.circle_filled(center + egui::vec2(2.0, -1.0), 4.5, bg_color);
}

fn draw_globe(painter: &egui::Painter, center: egui::Pos2, color: egui::Color32) {
    painter.circle_stroke(center, 5.5, egui::Stroke::new(1.0, color));
    // horizontal line
    painter.line_segment([center - egui::vec2(5.5, 0.0), center + egui::vec2(5.5, 0.0)], egui::Stroke::new(1.0, color));
    // vertical line
    painter.line_segment([center - egui::vec2(0.0, 5.5), center + egui::vec2(0.0, 5.5)], egui::Stroke::new(1.0, color));
}

fn system_btn(
    ui: &mut egui::Ui,
    icon: SystemIcon,
    label: &str,
    text_color: egui::Color32,
    is_dark: bool,
    sidebar_bg: egui::Color32,
) -> egui::Response {
    let frame = egui::Frame::new()
        .fill(egui::Color32::TRANSPARENT)
        .corner_radius(6)
        .inner_margin(egui::Margin::symmetric(12, 7))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                let (rect, _) = ui.allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::hover());
                let center = rect.center();
                let painter = ui.painter();

                match icon {
                    SystemIcon::SunMoon => {
                        if is_dark {
                            draw_sun(painter, center, text_color);
                        } else {
                            draw_moon(painter, center, text_color, sidebar_bg);
                        }
                    }
                    SystemIcon::Globe => {
                        draw_globe(painter, center, text_color);
                    }
                }

                ui.add_space(8.0);
                ui.label(egui::RichText::new(label).size(13.0).color(text_color));
            });
        });
    let id = ui.make_persistent_id(label);
    let response = ui.interact(frame.response.rect, id, egui::Sense::click());
    if response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    response
}

fn main() -> eframe::Result {
    env_logger::init();

    let db_path = "monstock.db";
    let conn = monstock_core::db::open(db_path).expect("Failed to open database");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_fullscreen(true)
            .with_min_inner_size([800.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "MonStock",
        options,
        Box::new(|cc| {
            style::setup_fonts(&cc.egui_ctx);
            let mut s = (*cc.egui_ctx.global_style()).clone();
            s.visuals.selection.stroke = egui::Stroke::new(1.0, ACCENT);
            s.visuals.selection.bg_fill = ACCENT_DIM;
            s.spacing.item_spacing = egui::vec2(8.0, 6.0);
            cc.egui_ctx.set_global_style(s);
            Ok(Box::new(MonStockApp::new(conn)))
        }),
    )
}
