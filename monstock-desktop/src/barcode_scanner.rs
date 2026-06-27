pub struct BarcodeInput {
    pub value: String,
    submitted: bool,
}

impl BarcodeInput {
    pub fn new() -> Self {
        Self { value: String::new(), submitted: false }
    }

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

pub fn scan_ui(ui: &mut egui::Ui, input: &mut BarcodeInput) -> bool {
    let mut triggered = false;
    ui.horizontal(|ui| {
        let resp = ui.add_sized(
            egui::vec2(ui.available_width(), 0.0),
            egui::TextEdit::singleline(&mut input.value)
                .desired_width(f32::INFINITY)
                .hint_text("🔍 Scan barcode or type"),
        );
        if (resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
            && !input.value.trim().is_empty()
        {
            input.submitted = true;
            triggered = true;
            input.value = input.value.trim().to_string();
        }
    });
    triggered
}
