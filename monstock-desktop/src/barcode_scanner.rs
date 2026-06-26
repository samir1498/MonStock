use crate::style::*;

pub struct BarcodeInput {
    pub value: String,
    submitted: bool,
}

impl BarcodeInput {
    pub fn new() -> Self {
        Self { value: String::new(), submitted: false }
    }

    /// Returns the scanned barcode if user submitted one this frame.
    pub fn take_scan(&mut self) -> Option<String> {
        if self.submitted {
            self.submitted = false;
            Some(self.value.clone())
        } else {
            None
        }
    }
}

impl Default for BarcodeInput {
    fn default() -> Self { Self::new() }
}

/// Renders a barcode text input + lookup button.
/// Returns true when the user triggers a scan (Enter or button click).
pub fn scan_ui(ui: &mut egui::Ui, input: &mut BarcodeInput, is_dark: bool) -> bool {
    let mut triggered = false;
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Barcode").size(13.0).color(text_color(is_dark)).strong());
        ui.add_space(4.0);
        let resp = ui.add(egui::TextEdit::singleline(&mut input.value)
            .desired_width(140.0)
            .hint_text("Scan or type barcode"));
        let lookup = btn(ui, egui::RichText::new("Lookup").size(12.0));
        if (lookup.clicked() || (resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))))
            && !input.value.trim().is_empty()
        {
            input.submitted = true;
            triggered = true;
            input.value = input.value.trim().to_string();
        }
    });
    triggered
}
