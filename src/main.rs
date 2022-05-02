use qrcode_png::{Color, QrCode, QrCodeEcc};

// buffer to base64 encode the qr code
fn buffer_to_base64(buffer: &[u8]) -> String {
    format!("data:image/png;base64,{}", base64::encode(buffer))
}

fn generate_qr_code(text: &str, ecc: QrCodeEcc, zoom: u32, margin: u32) -> String {
    let mut qr_code = QrCode::new(text.as_bytes(), ecc).unwrap();
    qr_code.zoom(zoom).margin(margin);
    
    let buf = qr_code.generate(Color::Bitmap(false, true)).unwrap();
    buffer_to_base64(&buf)
}


fn main() {
    let text = "Hello, world!";
    let result = generate_qr_code(text, QrCodeEcc::Medium, 10, 10);
    println!("{}", result);
}