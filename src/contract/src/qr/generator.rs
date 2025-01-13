use image::{ImageBuffer, Rgba};
use qrcode_generator::QrCodeEcc;
use std::io::Cursor;

use crate::models::qrcode::QrCodeOptions;

/// Generates a QR code image in PNG format for the given input text.
/// The requested image size should be specified in pixels.
pub fn generate(options: QrCodeOptions) -> Result<Vec<u8>, anyhow::Error> {
    // Generate a QR code image that can tolerate 25% of erroneous codewords.
    let mut qr = image::DynamicImage::ImageLuma8(qrcode_generator::to_image_buffer(
        options.link,
        QrCodeEcc::Quartile,
        options.size,
    )?)
    .into_rgba8();

    if options.gradient {
        add_gradient(&mut qr);
    }

    if options.transparent {
        make_transparent(&mut qr);
    }

    let mut result = vec![];
    qr.write_to(&mut Cursor::new(&mut result), image::ImageOutputFormat::Png)?;
    Ok(result)
}

/// Replaces white pixels in the image with transparent pixels.
fn make_transparent(qr: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    for (_x, _y, pixel) in qr.enumerate_pixels_mut() {
        if pixel.0 == [255, 255, 255, 255] {
            *pixel = image::Rgba([255, 255, 255, 0]);
        }
    }
}

/// Adds a color gradient to the black squares of the QR code image.
/// The gradient goes from the center of the image to its sides.
fn add_gradient(qr: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let image_size = qr.width().min(qr.height()) as usize;

    // Prepare a linear gradient function based on two colors.
    let gradient = colorgrad::CustomGradient::new()
        .colors(&[
            colorgrad::Color::from_rgba8(100, 0, 100, 255),
            colorgrad::Color::from_rgba8(30, 5, 60, 255),
        ])
        .build()
        .unwrap();

    // The gradient goes from the center of the image to its sides.
    let center = (image_size / 2) as u32;
    for (x, y, pixel) in qr.enumerate_pixels_mut() {
        if pixel.0 == [0, 0, 0, 255] {
            // Use a simple Manhattan distance as an estimate of how far the
            // pixel is from the center of the image.
            let distance = x.abs_diff(center) + y.abs_diff(center);
            let rgba = gradient.at(distance as f64 / image_size as f64).to_rgba8();
            *pixel = image::Rgba(rgba);
        }
    }
} 