use qrcode::render::svg;
use qrcode::QrCode;

pub fn qr(content: &str) -> String {
    let code = QrCode::new(content).unwrap();
    let image = code
        .render()
        .min_dimensions(400, 400)
        .max_dimensions(400, 400)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();
    image
}
