use egui;
use crate::i18n::{self, Lang};
use crate::style::*;

pub struct DashboardState;

impl Default for DashboardState {
    fn default() -> Self { Self }
}

enum ActivityType {
    Sale { title: String, time: String, amount: f64 },
    Expense { category: String, time: String, amount: f64 },
}

fn stat_card(ui: &mut egui::Ui, label: &str, value: &str, val_color: egui::Color32, delta: f64, delta_is_percentage: bool, is_dark: bool, lang: Lang) {
    let frame = egui::Frame::new()
        .fill(raised(is_dark))
        .stroke(egui::Stroke::new(1.0, border(is_dark)))
        .corner_radius(8)
        .inner_margin(egui::Margin::symmetric(18, 16));
    frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        ui.label(egui::RichText::new(label).size(13.0).color(text_dim_c(is_dark)).strong());
        ui.add_space(4.0);
        
        let display_color = if !is_dark && val_color == TEXT {
            egui::Color32::from_rgb(20, 20, 28)
        } else if !is_dark && val_color == ACCENT {
            egui::Color32::from_rgb(70, 110, 150)
        } else {
            val_color
        };
        
        ui.label(egui::RichText::new(value).size(26.0).color(display_color).strong());
        ui.add_space(4.0);
        
        if delta_is_percentage {
            let label_text = if delta >= 0.0 {
                format!("+{:.1}% {}", delta, i18n::t("vs_yesterday", lang))
            } else {
                format!("{:.1}% {}", delta, i18n::t("vs_yesterday", lang))
            };
            let color = if delta >= 0.0 { GOOD } else { BAD };
            ui.colored_label(color, egui::RichText::new(label_text).size(12.0).monospace());
        } else {
            let label_text = if delta >= 0.0 {
                format!("+{} {}", delta as i64, i18n::t("vs_yesterday", lang))
            } else {
                format!("{} {}", delta as i64, i18n::t("vs_yesterday", lang))
            };
            let color = if delta >= 0.0 { GOOD } else { BAD };
            ui.colored_label(color, egui::RichText::new(label_text).size(12.0).monospace());
        }
    });
}

fn quick_action(ui: &mut egui::Ui, sigil: &str, title: &str, desc: &str, is_dark: bool) -> egui::Response {
    let frame = egui::Frame::new()
        .fill(raised(is_dark))
        .stroke(egui::Stroke::new(1.0, border(is_dark)))
        .corner_radius(8)
        .inner_margin(egui::Margin::symmetric(16, 14));
    
    let response = frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        ui.horizontal(|ui| {
            let sigil_bg = if is_dark { SURFACE } else { egui::Color32::from_rgb(240, 240, 245) };
            egui::Frame::new()
                .fill(sigil_bg)
                .stroke(egui::Stroke::new(1.0, border(is_dark)))
                .corner_radius(6)
                .inner_margin(egui::Margin::symmetric(10, 8))
                .show(ui, |ui| {
                    ui.label(egui::RichText::new(sigil).size(16.0).color(text_color(is_dark)).strong().monospace());
                });
            
            ui.add_space(8.0);
            ui.vertical(|ui| {
                ui.label(egui::RichText::new(title).size(15.0).color(text_color(is_dark)).strong());
                ui.add_space(2.0);
                ui.label(egui::RichText::new(desc).size(13.0).color(text_dim_c(is_dark)));
            });
        });
    }).response;
    
    let res = ui.interact(response.rect, response.id, egui::Sense::click());
    if res.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    res
}

pub fn show(ui: &mut egui::Ui, conn: &mut diesel::SqliteConnection, lang: Lang, is_dark: bool, _state: &mut DashboardState) -> Option<crate::Screen> {
    page_header(ui, "#", i18n::t("dashboard", lang), i18n::t("overview", lang), is_dark);

    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let stats = monstock_core::services::dashboard_service::daily_stats(conn, &today).unwrap_or_default();
    let cost_total = monstock_core::services::sale_service::daily_cost_total(conn, &today).unwrap_or(0.0);
    let profit = monstock_core::services::dashboard_service::profit(stats.sales_total, cost_total, stats.expenses_total);
    let low_stock = monstock_core::services::dashboard_service::low_stock_products(conn, 5).unwrap_or_default();

    // yesterday stats for deltas
    let yesterday = (chrono::Local::now() - chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
    let stats_yesterday = monstock_core::services::dashboard_service::daily_stats(conn, &yesterday).unwrap_or_default();
    let cost_yesterday = monstock_core::services::sale_service::daily_cost_total(conn, &yesterday).unwrap_or(0.0);
    let profit_yesterday = monstock_core::services::dashboard_service::profit(stats_yesterday.sales_total, cost_yesterday, stats_yesterday.expenses_total);

    let sales_pct = if stats_yesterday.sales_total > 0.0 {
        (stats.sales_total - stats_yesterday.sales_total) / stats_yesterday.sales_total * 100.0
    } else {
        0.0
    };

    let expenses_pct = if stats_yesterday.expenses_total > 0.0 {
        (stats.expenses_total - stats_yesterday.expenses_total) / stats_yesterday.expenses_total * 100.0
    } else {
        0.0
    };

    let profit_pct = if profit_yesterday != 0.0 {
        (profit - profit_yesterday) / profit_yesterday.abs() * 100.0
    } else {
        0.0
    };

    let tx_diff = stats.transaction_count - stats_yesterday.transaction_count;

    let mut next_screen = None;

    // Alert banner
    if !low_stock.is_empty() {
        let alert_frame = egui::Frame::new()
            .fill(raised(is_dark))
            .stroke(egui::Stroke::new(1.0, border(is_dark)))
            .corner_radius(8)
            .inner_margin(egui::Margin::symmetric(16, 12));
        let resp = alert_frame.show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                let sigil_bg = if is_dark { SURFACE } else { egui::Color32::from_rgb(240, 240, 245) };
                egui::Frame::new()
                    .fill(sigil_bg)
                    .stroke(egui::Stroke::new(1.0, border(is_dark)))
                    .corner_radius(6)
                    .inner_margin(egui::Margin::symmetric(6, 4))
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new("!").size(13.0).monospace().color(WARN));
                    });
                ui.add_space(8.0);
                ui.label(egui::RichText::new(
                    format!("{} {} {}", low_stock.len(), i18n::t("items", lang), i18n::t("low_stock", lang))
                ).size(14.0).color(text_color(is_dark)).strong());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.colored_label(text_dim_c(is_dark), egui::RichText::new(i18n::t("view_inventory", lang)).size(13.0));
                });
            });
        }).response;
        
        let sense = ui.interact(resp.rect, resp.id, egui::Sense::click());
        if sense.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
        if sense.clicked() {
            next_screen = Some(crate::Screen::Products);
        }
        ui.add_space(16.0);
    }

    ui.add_space(8.0);

    // Stats row (4 horizontal cards)
    ui.columns(4, |cols| {
        stat_card(&mut cols[0], i18n::t("sales", lang), &format!("{:.0} DA", stats.sales_total), GOOD, sales_pct, true, is_dark, lang);
        stat_card(&mut cols[1], i18n::t("transactions", lang), &format!("{}", stats.transaction_count), ACCENT, tx_diff as f64, false, is_dark, lang);
        stat_card(&mut cols[2], i18n::t("expenses", lang), &format!("{:.0} DA", stats.expenses_total), BAD, expenses_pct, true, is_dark, lang);
        stat_card(&mut cols[3], i18n::t("profit", lang), &format!("{:.0} DA", profit), if profit >= 0.0 { GOOD } else { BAD }, profit_pct, true, is_dark, lang);
    });

    ui.add_space(20.0);

    // Left / Right columns
    ui.columns(2, |cols| {
        // Chart column (Sales by Hour)
        cols[0].vertical(|ui| {
            ui.label(egui::RichText::new(i18n::t("sales_by_hour", lang)).size(16.0).color(text_color(is_dark)).strong());
            ui.add_space(8.0);
            
            card(ui, is_dark, |ui| {
                ui.set_min_width(ui.available_width());
                ui.add_space(8.0);
                
                // group sales by hour
                let txs = monstock_core::services::sale_service::find_transactions_by_date(conn, &today).unwrap_or_default();
                let mut hourly_sales = vec![0.0f64; 24];
                for tx in &txs {
                    if let Some(time_part) = tx.timestamp.split('T').nth(1) {
                        if let Some(hour_str) = time_part.split(':').next() {
                            if let Ok(hour) = hour_str.parse::<usize>() {
                                if hour < 24 {
                                    hourly_sales[hour] += tx.total;
                                }
                            }
                        }
                    }
                }
                
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 8.0;
                    let max_amount = hourly_sales.iter().cloned().fold(0.0, f64::max);
                    
                    for hr in 8..20 {
                        let amount = hourly_sales[hr];
                        ui.vertical(|ui| {
                            let pct = if max_amount > 0.0 { (amount / max_amount) as f32 } else { 0.0 };
                            let bar_height = 100.0 * pct;
                            
                            let (rect, _) = ui.allocate_exact_size(egui::vec2(16.0, 100.0), egui::Sense::hover());
                            let p = ui.painter();
                            let bar_bg = if is_dark { RAISED } else { egui::Color32::from_rgb(240, 240, 245) };
                            p.rect_filled(rect, egui::CornerRadius::same(3), bar_bg);
                            
                            if bar_height > 0.0 {
                                let fill_rect = egui::Rect::from_min_max(
                                    egui::pos2(rect.left(), rect.bottom() - bar_height),
                                    rect.max
                                );
                                p.rect_filled(fill_rect, egui::CornerRadius::same(3), ACCENT);
                            }
                            
                            ui.add_space(4.0);
                            ui.colored_label(text_dim_c(is_dark), egui::RichText::new(format!("{:02}", hr)).size(11.0).monospace());
                        });
                    }
                });
            });
        });
        
        // Recent Activity column
        cols[1].vertical(|ui| {
            ui.label(egui::RichText::new(i18n::t("recent_activity", lang)).size(16.0).color(text_color(is_dark)).strong());
            ui.add_space(8.0);
            
            card(ui, is_dark, |ui| {
                ui.set_min_width(ui.available_width());
                
                // fetch activities
                let txs = monstock_core::services::sale_service::find_transactions_by_date(conn, &today).unwrap_or_default();
                let exps = monstock_core::repos::expense_repo::find_expenses_by_date_range(conn, &today, &today).unwrap_or_default();
                
                let mut activities = Vec::new();
                for tx in txs {
                    let items = monstock_core::services::sale_service::find_items_by_transaction(conn, tx.id).unwrap_or_default();
                    let title = if items.is_empty() {
                        i18n::t("sale_short", lang).to_string()
                    } else if items.len() == 1 {
                        format!("{} — {}", i18n::t("sale_short", lang), items[0].product_name)
                    } else {
                        format!("{} — {} {}", i18n::t("sale_short", lang), items.len(), i18n::t("items", lang))
                    };
                    let time = tx.timestamp.split('T').nth(1).unwrap_or("00:00:00").chars().take(5).collect::<String>();
                    activities.push((tx.timestamp.clone(), ActivityType::Sale { title, time, amount: tx.total }));
                }
                
                for exp in exps {
                    let time = exp.created_at.split('T').nth(1).unwrap_or("00:00:00").chars().take(5).collect::<String>();
                    activities.push((exp.created_at.clone(), ActivityType::Expense { category: exp.category, time, amount: exp.amount }));
                }
                
                activities.sort_by(|a, b| b.0.cmp(&a.0));
                
                if activities.is_empty() {
                    ui.colored_label(text_dim_c(is_dark), egui::RichText::new(i18n::t("no_activity", lang)).size(13.0));
                } else {
                    for (_, act) in activities.iter().take(5) {
                        ui.horizontal(|ui| {
                            match act {
                                ActivityType::Sale { title, time, amount } => {
                                    let icon_bg = if is_dark { RAISED } else { egui::Color32::from_rgb(240, 240, 245) };
                                    egui::Frame::new().fill(icon_bg).stroke(egui::Stroke::new(1.0, border(is_dark))).corner_radius(4).inner_margin(4).show(ui, |ui| {
                                        ui.label(egui::RichText::new("$").size(12.0).color(GOOD).monospace().strong());
                                    });
                                    ui.vertical(|ui| {
                                        ui.label(egui::RichText::new(title).size(14.0).color(text_color(is_dark)).strong());
                                        ui.label(egui::RichText::new(time).size(12.0).color(text_dim_c(is_dark)));
                                    });
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.colored_label(GOOD, egui::RichText::new(format!("+{:.0} DA", amount)).size(14.0).monospace().strong());
                                    });
                                }
                                ActivityType::Expense { category, time, amount } => {
                                    let icon_bg = if is_dark { RAISED } else { egui::Color32::from_rgb(240, 240, 245) };
                                    egui::Frame::new().fill(icon_bg).stroke(egui::Stroke::new(1.0, border(is_dark))).corner_radius(4).inner_margin(4).show(ui, |ui| {
                                        ui.label(egui::RichText::new("E").size(12.0).color(BAD).monospace().strong());
                                    });
                                    ui.vertical(|ui| {
                                        ui.label(egui::RichText::new(format!("{} — {}", i18n::t("expense_short", lang), category)).size(14.0).color(text_color(is_dark)).strong());
                                        ui.label(egui::RichText::new(time).size(12.0).color(text_dim_c(is_dark)));
                                    });
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.colored_label(BAD, egui::RichText::new(format!("-{:.0} DA", amount)).size(14.0).monospace().strong());
                                    });
                                }
                            }
                        });
                        ui.add_space(4.0);
                        ui.separator();
                        ui.add_space(4.0);
                    }
                }
            });
        });
    });

    ui.add_space(20.0);

    // Quick actions
    ui.label(egui::RichText::new(i18n::t("quick_actions", lang)).size(16.0).color(text_color(is_dark)).strong());
    ui.add_space(8.0);
    
    ui.columns(3, |cols| {
        if quick_action(&mut cols[0], "$", i18n::t("new_sale", lang), i18n::t("new_sale_desc", lang), is_dark).clicked() {
            next_screen = Some(crate::Screen::Sales);
        }
        if quick_action(&mut cols[1], "E", i18n::t("new_expense", lang), i18n::t("new_expense_desc", lang), is_dark).clicked() {
            next_screen = Some(crate::Screen::Expenses);
        }
        if quick_action(&mut cols[2], "B", i18n::t("receive_stock", lang), i18n::t("receive_stock_desc", lang), is_dark).clicked() {
            next_screen = Some(crate::Screen::PurchaseOrders);
        }
    });

    ui.add_space(20.0);
    
    // Low stock section
    ui.label(egui::RichText::new(i18n::t("low_stock", lang)).size(16.0).color(text_color(is_dark)).strong());
    ui.add_space(8.0);

    card(ui, is_dark, |ui| {
        ui.set_min_width(ui.available_width());
        if low_stock.is_empty() {
            ui.colored_label(text_dim_c(is_dark), egui::RichText::new(i18n::t("all_stocked", lang)).size(14.0));
        } else {
            egui::Grid::new("low_stock_grid")
                .striped(true)
                .min_col_width(120.0)
                .show(ui, |ui| {
                    ui.colored_label(text_sec_color(is_dark), egui::RichText::new(i18n::t("name", lang)).strong());
                    ui.colored_label(text_sec_color(is_dark), egui::RichText::new(i18n::t("stock", lang)).strong());
                    ui.colored_label(text_sec_color(is_dark), egui::RichText::new(i18n::t("price", lang)).strong());
                    ui.end_row();

                    for p in &low_stock {
                        ui.label(egui::RichText::new(&p.name).size(14.0).color(text_color(is_dark)));
                        ui.colored_label(BAD, egui::RichText::new(format!("{}", p.quantity_on_hand)).size(14.0));
                        ui.label(egui::RichText::new(format!("{:.0} DA", p.selling_price)).size(14.0).color(text_color(is_dark)));
                        ui.end_row();
                    }
                });
        }
    });
        
    next_screen
}
