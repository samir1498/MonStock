use egui_extras::image::load_svg_bytes_with_size;

fn tinted_svg(svg_bytes: &[u8], color: egui::Color32) -> Vec<u8> {
    let hex = format!("#{:02x}{:02x}{:02x}", color.r(), color.g(), color.b());
    let text = std::str::from_utf8(svg_bytes).unwrap();
    text.replace("currentColor", &hex).into_bytes()
}

fn load_icon(ctx: &egui::Context, name: &str, bytes: &[u8]) -> egui::TextureHandle {
    let tinted = tinted_svg(bytes, egui::Color32::WHITE);
    let options = Default::default();
    let size_hint = egui::SizeHint::Height(24);
    let image = load_svg_bytes_with_size(&tinted, size_hint, &options).unwrap_or_else(|_| panic!("SVG icon {name}"));
    ctx.load_texture(name, image, Default::default())
}

pub struct Icons {
    pub sun: egui::TextureHandle,
    pub moon: egui::TextureHandle,
    pub globe: egui::TextureHandle,
}

impl Icons {
    pub fn new(ctx: &egui::Context) -> Self {
        Self {
            sun: load_icon(ctx, "sun", include_bytes!("../icons/sun.svg")),
            moon: load_icon(ctx, "moon", include_bytes!("../icons/moon.svg")),
            globe: load_icon(ctx, "globe", include_bytes!("../icons/globe.svg")),
        }
    }
}
