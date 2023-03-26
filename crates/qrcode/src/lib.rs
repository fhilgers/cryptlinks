use fast_qr::{
    convert::{image::ImageBuilder, svg::SvgBuilder, Builder, Shape},
    qr::QRBuilder,
};

pub fn save_svg(payload: &str, path: &str) {
    let qrcode = QRBuilder::new(payload).build().unwrap();

    let _svg = SvgBuilder::default()
        .shape(Shape::Square)
        .margin(0)
        .to_file(&qrcode, path)
        .unwrap();
}

pub fn save_png(payload: &str, path: &str) {
    let qrcode = QRBuilder::new(payload).build().unwrap();

    let _img = ImageBuilder::default()
        .shape(Shape::Square)
        .background_color([255, 255, 255, 255])
        .fit_width(400)
        .margin(10)
        .to_file(&qrcode, path)
        .unwrap();
}

pub fn as_str(payload: &str) -> String {
    let qrcode = QRBuilder::new(payload).build().unwrap();

    qrcode.to_str()
}
