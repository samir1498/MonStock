use std::path::PathBuf;
use egui;
use crate::i18n::{self, Lang};
use crate::style::*;
use crate::backup;

#[derive(Default)]
pub struct SettingsState {
    pub feedback: Option<String>,
    pub feedback_is_error: bool,
}

fn open_file_manager(path: &PathBuf) {
    #[cfg(target_os = "linux")]
    let _ = std::process::Command::new("xdg-open").arg(path).spawn();
    #[cfg(target_os = "macos")]
    let _ = std::process::Command::new("open").arg(path).spawn();
    #[cfg(target_os = "windows")]
    let _ = std::process::Command::new("explorer").arg(path).spawn();
}

pub fn show(ui: &mut egui::Ui, lang: Lang, is_dark: bool, state: &mut SettingsState, db_path: &str) {
    page_header(ui, "*", i18n::t("system", lang), "", is_dark);

    // Backup section
    ui.label(egui::RichText::new("Backups").size(16.0).color(text_color(is_dark)).strong());
    ui.add_space(8.0);

    card(ui, is_dark, |ui| {
        ui.set_min_width(ui.available_width());

        let backup_path = backup::backup_dir();
        let path_str = backup_path.to_string_lossy().to_string();

        ui.label(egui::RichText::new("Location").size(13.0).color(text_dim_c(is_dark)).strong());
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(&path_str).size(12.0).color(text_color(is_dark)).monospace());
            ui.add_space(8.0);
            if btn(ui, egui::RichText::new(i18n::t("open", lang)).size(12.0)).clicked() {
                open_file_manager(&backup_path);
            }
        });

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);

        let last = backup::last_backup_date().unwrap_or_else(|| i18n::t("never", lang).to_string());
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Last backup").size(13.0).color(text_dim_c(is_dark)).strong());
            ui.add_space(8.0);
            ui.label(egui::RichText::new(&last).size(12.0).color(text_color(is_dark)));
            ui.add_space(12.0);
            if primary_btn(ui, &format!("{} {}", i18n::t("create", lang), i18n::t("backup", lang))).clicked() {
                match backup::create_backup(db_path) {
                    Ok(p) => {
                        state.feedback = Some(format!("{}: {}", i18n::t("backup_created", lang), p.file_name().unwrap_or_default().to_string_lossy()));
                        state.feedback_is_error = false;
                    }
                    Err(e) => {
                        state.feedback = Some(format!("{}: {}", i18n::t("error", lang), e));
                        state.feedback_is_error = true;
                    }
                }
            }
        });

        if let Some(ref fb) = state.feedback {
            ui.add_space(4.0);
            let fc = if state.feedback_is_error { BAD } else { GOOD };
            ui.colored_label(fc, egui::RichText::new(fb).size(12.0));
        }

        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);

        // List existing backups
        if let Ok(entries) = std::fs::read_dir(&backup_path) {
            let mut backup_files: Vec<_> = entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    let name = e.file_name().to_string_lossy().to_string();
                    name.starts_with("monstock_") && name.ends_with(".db")
                })
                .collect();
            backup_files.sort_by_key(|e| e.file_name());
            backup_files.reverse();

            if backup_files.is_empty() {
                ui.colored_label(text_dim_c(is_dark), egui::RichText::new(i18n::t("no_backups", lang)).size(13.0));
            } else {
                ui.label(egui::RichText::new("Saved backups").size(13.0).color(text_dim_c(is_dark)).strong());
                ui.add_space(4.0);
                for entry in backup_files.iter().take(10) {
                    let name = entry.file_name().to_string_lossy().to_string();
                    let date = name.trim_start_matches("monstock_").trim_end_matches(".db");
                    let size = std::fs::metadata(entry.path()).map(|m| format!("{:.1} KB", m.len() as f64 / 1024.0)).unwrap_or_default();
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(date).size(12.0).color(text_color(is_dark)).monospace());
                        ui.add_space(8.0);
                        ui.colored_label(text_dim_c(is_dark), egui::RichText::new(size).size(11.0).monospace());
                    });
                }
                if backup_files.len() > 10 {
                    ui.add_space(2.0);
                    ui.colored_label(text_dim_c(is_dark), egui::RichText::new(format!("+ {} more", backup_files.len() - 10)).size(11.0));
                }
            }
        } else {
            ui.colored_label(text_dim_c(is_dark), egui::RichText::new(i18n::t("no_backups", lang)).size(13.0));
        }
    });
}
