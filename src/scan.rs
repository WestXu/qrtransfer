use super::log;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, RgbImage, RgbaImage};
use quircs::Quirc;

pub fn scan(width: u32, height: u32, data: Vec<u8>) -> Vec<String> {
    let img: RgbaImage = ImageBuffer::from_raw(width, height, data).unwrap();

    // convert to gray scale
    let img_gray = DynamicImage::ImageRgba8(img).into_luma();

    // create a decoder
    let mut decoder = Quirc::default();

    // identify all qr codes
    let codes = decoder.identify(
        img_gray.width() as usize,
        img_gray.height() as usize,
        &img_gray,
    );

    codes
        .into_iter()
        .map(|code| {
            if let Ok(code) = code {
                if let Ok(decoded) = code.decode() {
                    match String::from_utf8(decoded.payload) {
                        Ok(msg) => Some(msg),
                        Err(_) => None,
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .filter(|msg| msg.is_some())
        .map(|msg| msg.unwrap())
        .collect()
}
