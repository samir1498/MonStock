#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 700.0])
            .with_min_inner_size([800.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "MonStock",
        options,
        Box::new(|_cc| {
            // TODO: Initialize database and app state
            Ok(Box::new(MonStockApp::default()))
        }),
    )
}

use egui;

struct MonStockApp {
    greeting: String,
}

impl Default for MonStockApp {
    fn default() -> Self {
        Self {
            greeting: "MonStock Desktop".to_string(),
        }
    }
}

impl eframe::App for MonStockApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading(&self.greeting);
        ui.label("Welcome to MonStock - Inventory Management System");
    }
}