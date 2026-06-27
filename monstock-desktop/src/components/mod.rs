pub mod data_table;

use crate::i18n::{self, Lang};
use crate::style::*;
use diesel::SqliteConnection;

pub use data_table::{ColumnDef, ColumnSizing, DataTable};

pub trait ModalScreen {
    fn save(&mut self, lang: Lang, conn: &mut SqliteConnection) -> bool;
    fn close_modal(&mut self);
}

/// Renders a modal window with standard boilerplate.
pub fn modal_window<R>(
    ctx: &egui::Context,
    title: &str,
    size: [f32; 2],
    add_contents: R,
)
where
    R: FnOnce(&mut egui::Ui),
{
    egui::Window::new(title)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .fixed_size(size)
        .collapsible(false)
        .title_bar(true)
        .resizable(false)
        .movable(false)
        .show(ctx, |ui| {
            egui::Frame::new()
                .inner_margin(egui::Margin::symmetric(20, 16))
                .show(ui, add_contents);
        });
}

/// Renders save/cancel buttons + error text for modal forms.
pub fn modal_actions(
    ui: &mut egui::Ui,
    lang: Lang,
    error: Option<&str>,
    conn: &mut SqliteConnection,
    screen: &mut dyn ModalScreen,
) {
    ui.add_space(12.0);
    if let Some(err) = error {
        ui.colored_label(BAD, err);
        ui.add_space(4.0);
    }
    ui.horizontal(|ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if btn(ui, i18n::t("save", lang)).clicked() { screen.save(lang, conn); }
            ui.add_space(8.0);
            if btn(ui, i18n::t("cancel", lang)).clicked() { screen.close_modal(); }
        });
    });
}

/// Standard delete X button with consistent styling.
pub fn delete_btn(ui: &mut egui::Ui, is_dark: bool) -> egui::Response {
    btn_custom(ui, egui::Button::new(
        egui::RichText::new("X").size(10.0).color(text_dim_c(is_dark))
    ).fill(egui::Color32::TRANSPARENT).min_size(egui::vec2(20.0, 20.0)))
}
