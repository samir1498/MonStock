pub const BG: egui::Color32 = egui::Color32::from_rgb(10, 10, 12);
pub const SURFACE: egui::Color32 = egui::Color32::from_rgb(17, 17, 20);
pub const RAISED: egui::Color32 = egui::Color32::from_rgb(22, 22, 26);
pub const BORDER: egui::Color32 = egui::Color32::from_rgb(30, 30, 38);
pub const BORDER_STRONG: egui::Color32 = egui::Color32::from_rgb(42, 42, 52);
pub const TEXT: egui::Color32 = egui::Color32::from_rgb(240, 240, 245);
pub const TEXT_SEC: egui::Color32 = egui::Color32::from_rgb(160, 165, 180);
pub const TEXT_DIM: egui::Color32 = egui::Color32::from_rgb(115, 120, 135);
pub const ACCENT: egui::Color32 = egui::Color32::from_rgb(140, 160, 185);
pub const ACCENT_DIM: egui::Color32 = egui::Color32::from_rgba_premultiplied(140, 160, 185, 45);
pub const GOOD: egui::Color32 = egui::Color32::from_rgb(120, 160, 140);
pub const BAD: egui::Color32 = egui::Color32::from_rgb(170, 120, 120);
pub const WARN: egui::Color32 = egui::Color32::from_rgb(160, 140, 120);

pub fn raised(is_dark: bool) -> egui::Color32 {
    if is_dark { RAISED } else { egui::Color32::from_rgb(255, 255, 255) }
}
pub fn border(is_dark: bool) -> egui::Color32 {
    if is_dark { BORDER } else { egui::Color32::from_rgb(225, 225, 230) }
}
pub fn text_dim_c(is_dark: bool) -> egui::Color32 {
    if is_dark { TEXT_DIM } else { egui::Color32::from_rgb(150, 150, 160) }
}
pub fn text_color(is_dark: bool) -> egui::Color32 {
    if is_dark { TEXT } else { egui::Color32::from_rgb(20, 20, 28) }
}
pub fn text_sec_color(is_dark: bool) -> egui::Color32 {
    if is_dark { TEXT_SEC } else { egui::Color32::from_rgb(70, 75, 90) }
}

pub fn tag(ui: &mut egui::Ui, label: &str, color: egui::Color32, is_dark: bool) {
    let text_color = if is_dark {
        color
    } else {
        egui::Color32::from_rgb(
            (color.r() as f32 * 0.35) as u8,
            (color.g() as f32 * 0.35) as u8,
            (color.b() as f32 * 0.35) as u8,
        )
    };
    let bg = egui::Color32::from_rgba_premultiplied(
        (color.r() as u32 * if is_dark { 20 } else { 40 } / 255) as u8,
        (color.g() as u32 * if is_dark { 20 } else { 40 } / 255) as u8,
        (color.b() as u32 * if is_dark { 20 } else { 40 } / 255) as u8,
        if is_dark { 20 } else { 40 },
    );
    let bc = egui::Color32::from_rgba_premultiplied(
        (color.r() as u32 * if is_dark { 64 } else { 120 } / 255) as u8,
        (color.g() as u32 * if is_dark { 64 } else { 120 } / 255) as u8,
        (color.b() as u32 * if is_dark { 64 } else { 120 } / 255) as u8,
        if is_dark { 64 } else { 120 },
    );
    egui::Frame::new().fill(bg).stroke(egui::Stroke::new(1.0, bc))
        .corner_radius(4).inner_margin(egui::Margin::symmetric(8, 2))
        .show(ui, |ui| { ui.label(egui::RichText::new(label).size(10.5).color(text_color).strong()); });
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
        ui.label(egui::RichText::new(icon).size(20.0).color(ACCENT).monospace());
        ui.add_space(10.0);
        ui.vertical(|ui| {
            ui.label(egui::RichText::new(title).size(20.0).color(text_color(is_dark)).strong());
            ui.add_space(2.0);
            ui.label(egui::RichText::new(subtitle).size(12.0).color(text_dim_c(is_dark)));
        });
    });
    ui.add_space(16.0);
}

pub fn primary_btn(ui: &mut egui::Ui, label: &str) -> egui::Response {
    let r = ui.add(egui::Button::new(egui::RichText::new(label).color(BG).size(12.0))
        .fill(TEXT).corner_radius(6).min_size(egui::vec2(80.0, 28.0)));
    if r.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    r
}

pub fn btn(ui: &mut egui::Ui, text: impl Into<egui::WidgetText>) -> egui::Response {
    let r = ui.button(text);
    if r.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    r
}

pub fn btn_custom(ui: &mut egui::Ui, button: egui::Button) -> egui::Response {
    let r = ui.add(button);
    if r.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    r
}

#[derive(Clone)]
pub struct SortState {
    pub column: Option<usize>,
    pub ascending: bool,
}

impl Default for SortState {
    fn default() -> Self {
        Self { column: None, ascending: true }
    }
}

pub fn compare_str(a: &str, b: &str, asc: bool) -> std::cmp::Ordering {
    if asc { a.cmp(b) } else { b.cmp(a) }
}

pub fn compare_float(a: f64, b: f64, asc: bool) -> std::cmp::Ordering {
    if asc { a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal) }
    else { b.partial_cmp(&a).unwrap_or(std::cmp::Ordering::Equal) }
}

pub fn compare_int(a: i32, b: i32, asc: bool) -> std::cmp::Ordering {
    if asc { a.cmp(&b) } else { b.cmp(&a) }
}

pub fn table_header(ui: &mut egui::Ui, label: &str) {
    ui.colored_label(TEXT_SEC, egui::RichText::new(label).size(12.5).strong());
}

pub fn sortable_header(ui: &mut egui::Ui, label: &str, idx: usize, sort: &mut SortState) -> bool {
    let is_active = sort.column == Some(idx);
    let arrow = if is_active {
        if sort.ascending { " ▲" } else { " ▼" }
    } else {
        ""
    };
    let text = format!("{}{}", label, arrow);
    let color = if is_active { TEXT } else { TEXT_SEC };
    let r = ui.add(egui::Label::new(
        egui::RichText::new(text).size(12.5).color(color).strong()
    ).sense(egui::Sense::click()));
    if r.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    if r.clicked() {
        if sort.column == Some(idx) {
            sort.ascending = !sort.ascending;
        } else {
            sort.column = Some(idx);
            sort.ascending = true;
        }
        return true;
    }
    false
}

pub fn mono_value(ui: &mut egui::Ui, value: &str, color: egui::Color32) {
    ui.label(egui::RichText::new(value).size(11.5).color(color).monospace());
}

pub fn amount_text(ui: &mut egui::Ui, value: &str, color: egui::Color32) {
    ui.label(egui::RichText::new(value).size(12.0).color(color).monospace());
}

pub struct PaginationState {
    pub page: i64,
    pub per_page: i64,
    pub total: i64,
}

impl PaginationState {
    pub fn total_pages(&self) -> i64 {
        if self.total == 0 { 1 } else { (self.total - 1) / self.per_page + 1 }
    }

    pub fn offset(&self) -> i64 {
        (self.page - 1).max(0) * self.per_page
    }

    pub fn next(&mut self) {
        if self.page < self.total_pages() { self.page += 1; }
    }

    pub fn prev(&mut self) {
        if self.page > 1 { self.page -= 1; }
    }

    pub fn visible_range(&self) -> (i64, i64) {
        let start = self.offset() + 1;
        let end = (self.offset() + self.per_page).min(self.total);
        (start, end)
    }
}

impl Default for PaginationState {
    fn default() -> Self {
        Self { page: 1, per_page: 10, total: 0 }
    }
}

pub fn pagination_ui(
    ui: &mut egui::Ui,
    pagination: &mut PaginationState,
    is_dark: bool,
) {
    let total_pages = pagination.total_pages();
    if pagination.total == 0 { return; }
    if total_pages <= 1 && pagination.total <= pagination.per_page { return; }

    ui.add_space(4.0);
    ui.separator();
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        let (start, end) = pagination.visible_range();
        let label = format!("{}–{} / {}", start, end, pagination.total);
        ui.colored_label(text_sec_color(is_dark), egui::RichText::new(label).size(14.0).monospace().strong());
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            let disabled = pagination.page <= 1;
            let btn_color = if disabled { text_dim_c(is_dark) } else { text_sec_color(is_dark) };
            let bg = if disabled { egui::Color32::TRANSPARENT } else { raised(is_dark) };
            let r = btn_custom(ui, egui::Button::new(egui::RichText::new("<").size(16.0).color(btn_color).strong())
                .fill(bg).stroke(egui::Stroke::new(1.0, border(is_dark))).corner_radius(4).min_size(egui::vec2(32.0, 28.0)));
            if !disabled && r.clicked() { pagination.prev(); }

            let disabled = pagination.page >= total_pages;
            let btn_color = if disabled { text_dim_c(is_dark) } else { text_sec_color(is_dark) };
            let bg = if disabled { egui::Color32::TRANSPARENT } else { raised(is_dark) };
            let r = btn_custom(ui, egui::Button::new(egui::RichText::new(">").size(16.0).color(btn_color).strong())
                .fill(bg).stroke(egui::Stroke::new(1.0, border(is_dark))).corner_radius(4).min_size(egui::vec2(32.0, 28.0)));
            if !disabled && r.clicked() { pagination.next(); }
        });
    });
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
