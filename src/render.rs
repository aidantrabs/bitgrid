use image::{GrayImage, Luma};
use std::path::Path;

pub fn save_png(history: &[Vec<u8>], path: &Path, scale: u32) {
    let height = history.len() as u32;
    let width = history[0].len() as u32;
    let mut img = GrayImage::new(width * scale, height * scale);

    for (y, row) in history.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let pixel = if cell == 1 { Luma([0u8]) } else { Luma([255u8]) };
            for dy in 0..scale {
                for dx in 0..scale {
                    img.put_pixel(x as u32 * scale + dx, y as u32 * scale + dy, pixel);
                }
            }
        }
    }

    img.save(path).expect("failed to save image");
}
