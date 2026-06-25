use egui;

pub const BG: egui::Color32 = egui::Color32::from_rgb(10, 10, 12);
pub const SURFACE: egui::Color32 = egui::Color32::from_rgb(17, 17, 20);
pub const RAISED: egui::Color32 = egui::Color32::from_rgb(22, 22, 26);
pub const HOVER: egui::Color32 = egui::Color32::from_rgb(28, 28, 34);
pub const BORDER: egui::Color32 = egui::Color32::from_rgb(30, 30, 38);
pub const BORDER_STRONG: egui::Color32 = egui::Color32::from_rgb(42, 42, 52);
pub const TEXT: egui::Color32 = egui::Color32::from_rgb(232, 232, 236);
pub const TEXT_SEC: egui::Color32 = egui::Color32::from_rgb(110, 110, 122);
pub const TEXT_DIM: egui::Color32 = egui::Color32::from_rgb(70, 70, 80);
pub const ACCENT: egui::Color32 = egui::Color32::from_rgb(124, 143, 163);
pub const ACCENT_DIM: egui::Color32 = egui::Color32::from_rgba_premultiplied(124, 143, 163, 40);
pub const GOOD: egui::Color32 = egui::Color32::from_rgb(106, 138, 122);
pub const BAD: egui::Color32 = egui::Color32::from_rgb(138, 106, 106);
pub const WARN: egui::Color32 = egui::Color32::from_rgb(138, 122, 106);

pub fn surface(is_dark: bool) -> egui::Color32 {
    if is_dark { SURFACE } else { egui::Color32::from_rgb(245, 245, 250) }
}
pub fn raised(is_dark: bool) -> egui::Color32 {
    if is_dark { RAISED } else { egui::Color32::from_rgb(255, 255, 255) }
}
pub fn border(is_dark: bool) -> egui::Color32 {
    if is_dark { BORDER } else { egui::Color32::from_rgb(225, 225, 230) }
}
pub fn text_dim_c(is_dark: bool) -> egui::Color32 {
    if is_dark { TEXT_DIM } else { egui::Color32::from_rgb(150, 150, 160) }
}

pub fn tag(ui: &mut egui::Ui, label: &str, color: egui::Color32) {
    let bg = egui::Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 20);
    let bc = egui::Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), 64);
    egui::Frame::new().fill(bg).stroke(egui::Stroke::new(1.0, bc))
        .corner_radius(4).inner_margin(egui::Margin::symmetric(8, 2))
        .show(ui, |ui| { ui.label(egui::RichText::new(label).size(10.5).color(color).strong()); });
}

pub fn stock_tag(ui: &mut egui::Ui, qty: i32) {
    let (l, c) = if qty == 0 { ("Out", BAD) } else if qty <= 10 { ("Low", WARN) } else { ("In Stock", GOOD) };
    tag(ui, l, c);
}

pub fn stock_bar(ui: &mut egui::Ui, qty: i32, max: i32) {
    let pct = if max > 0 { (qty as f32 / max as f32).clamp(0.0, 1.0) } else { 0.0 };
    let fc = if qty == 0 { BAD } else if qty <= 5 { WARN } else { TEXT_SEC };
    let (rect, _) = ui.allocate_exact_size(egui::vec2(40.0, 3.0), egui::Sense::hover());
    let p = ui.painter();
    p.rect_filled(rect, egui::CornerRadius::same(2), RAISED);
    p.rect_filled(
        egui::Rect::from_min_size(rect.min, egui::vec2(rect.width() * pct, rect.height())),
        egui::CornerRadius::same(2), fc);
}

pub fn card(ui: &mut egui::Ui, is_dark: bool, add_contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new().fill(raised(is_dark))
        .stroke(egui::Stroke::new(1.0, border(is_dark)))
        .corner_radius(8).inner_margin(egui::Margin::symmetric(18, 14))
        .show(ui, add_contents);
}

pub fn page_header(ui: &mut egui::Ui, icon: &str, title: &str, subtitle: &str, is_dark: bool) {
    ui.add_space(16.0);
    ui.horizontal(|ui| {
        ui.add_space(24.0);
        ui.label(egui::RichText::new(icon).size(20.0).color(ACCENT).monospace());
        ui.add_space(10.0);
        ui.vertical(|ui| {
            ui.label(egui::RichText::new(title).size(20.0).strong());
            ui.add_space(2.0);
            ui.label(egui::RichText::new(subtitle).size(12.0).color(text_dim_c(is_dark)));
        });
    });
    ui.add_space(16.0);
}

pub fn primary_btn(ui: &mut egui::Ui, label: &str) -> egui::Response {
    ui.add(egui::Button::new(egui::RichText::new(label).color(BG).size(12.0))
        .fill(TEXT).corner_radius(6).min_size(egui::vec2(80.0, 28.0)))
}

pub fn table_header(ui: &mut egui::Ui, label: &str) {
    ui.colored_label(TEXT_DIM, egui::RichText::new(label).size(10.5).strong());
}

pub fn mono_value(ui: &mut egui::Ui, value: &str, color: egui::Color32) {
    ui.label(egui::RichText::new(value).size(11.5).color(color).monospace());
}

pub fn amount_text(ui: &mut egui::Ui, value: &str, color: egui::Color32) {
    ui.label(egui::RichText::new(value).size(12.0).color(color).monospace());
}

pub fn product_avatar(ui: &mut egui::Ui, initials: &str) {
    egui::Frame::new().fill(RAISED).stroke(egui::Stroke::new(1.0, BORDER))
        .corner_radius(6).inner_margin(egui::Margin::symmetric(6, 4))
        .show(ui, |ui| { ui.label(egui::RichText::new(initials).size(11.0).color(TEXT_DIM).monospace()); });
}

pub fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert("OpenSans-Regular".into(),
        egui::FontData::from_static(include_bytes!("../fonts/OpenSans-Regular.ttf")).into());
    fonts.font_data.insert("OpenSans-Bold".into(),
        egui::FontData::from_static(include_bytes!("../fonts/OpenSans-Bold.ttf")).into());
    fonts.font_data.insert("JetBrainsMono-Regular".into(),
        egui::FontData::from_static(include_bytes!("../fonts/JetBrainsMono-Regular.ttf")).into());
    fonts.font_data.insert("JetBrainsMono-Bold".into(),
        egui::FontData::from_static(include_bytes!("../fonts/JetBrainsMono-Bold.ttf")).into());

    if let Some(p) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        p.clear();
        p.push("OpenSans-Regular".to_string());
        p.push("OpenSans-Bold".to_string());
    }
    if let Some(m) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
        m.clear();
        m.push("JetBrainsMono-Regular".to_string());
        m.push("JetBrainsMono-Bold".to_string());
    }

    ctx.set_fonts(fonts);
}
