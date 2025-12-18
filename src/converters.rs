use image::RgbaImage;

use crate::{
    color::{Color16, Color32},
    plane::Plane,
};

pub fn convert_posterize(image: &RgbaImage) -> Plane<Color16> {
    let mut result = Plane::new(image.width(), image.height());

    for (x, y, color) in image.enumerate_pixels() {
        result.set(x, y, Color16::from(Color32::from(color)));
    }

    result
}

pub fn convert_fs(image: &RgbaImage) -> Plane<Color16> {
    let mut inner = Plane::new(image.width(), image.height());
    let mut result = Plane::new(image.width(), image.height());
    for (x, y, color) in image.enumerate_pixels() {
        let original_color = Color32::from(color);
        let correction: Color32 = inner.get(x, y);
        let old_color = Color32::new(
            ((original_color.r as f64 + correction.r as f64 / 16.0) as i32).clamp(0, 255),
            ((original_color.g as f64 + correction.g as f64 / 16.0) as i32).clamp(0, 255),
            ((original_color.b as f64 + correction.b as f64 / 16.0) as i32).clamp(0, 255),
            original_color.a,
        );
        let new_color = Color16::from(old_color);
        result.set(x, y, new_color);
        let error = old_color - Color32::from(new_color);
        if x < image.width() - 1 {
            inner.add(x + 1, y, error * 7);
        }
        if y < image.height() - 1 {
            if x > 0 {
                inner.add(x - 1, y + 1, error * 3);
            }
            inner.add(x, y + 1, error * 5);
            if x < image.width() - 1 {
                inner.add(x + 1, y + 1, error);
            }
        }
    }
    result
}

const BAYER_INT_4X4: [i32; 16] = [
    0, 8, 2, 10, //
    12, 4, 14, 6, //
    3, 11, 1, 9, //
    15, 7, 13, 5, //
];

const BAYER_INT_8X8: [i32; 64] = [
    0, 32, 8, 40, 2, 34, 10, 42, //
    48, 16, 56, 24, 50, 18, 58, 26, //
    12, 44, 4, 36, 14, 46, 6, 38, //
    60, 28, 52, 20, 62, 30, 54, 22, //
    3, 35, 11, 43, 1, 33, 9, 41, //
    51, 19, 59, 27, 49, 17, 57, 25, //
    15, 47, 7, 39, 13, 45, 5, 37, //
    63, 31, 55, 23, 61, 29, 53, 21, //
];

const fn for_bayer4() -> [f64; 64] {
    let mut result = [0.0; 64];

    let mut y = 0usize;
    while y < 8 {
        let mut x = 0usize;
        while x < 8 {
            let ix = x % 4;
            let iy = y % 4;
            let iind = ix + iy * 4;
            let ind = x + y * 8;
            result[ind] = (BAYER_INT_4X4[iind] as f64) / 16.0 - 0.5;
            x += 1
        }
        y += 1;
    }

    result
}

const fn for_bayer8() -> [f64; 64] {
    let mut result = [0.0; 64];

    let mut i = 0usize;
    while i < 64 {
        result[i] = (BAYER_INT_8X8[i] as f64) / 64.0 - 0.5;
        i += 1;
    }

    result
}

const BAYER_FLOAT_4X4: [f64; 64] = for_bayer4();

const BAYER_FLOAT_8X8: [f64; 64] = for_bayer8();

const COLOR_RADIUS: f64 = 255.0 / 31.0;

fn get_wrapped(pattern: &[f64; 64], x: u32, y: u32) -> f64 {
    let x = (x % 8) as usize;
    let y = (y % 8) as usize;
    pattern[x + y * 8]
}

fn ordered_dithering(image: &RgbaImage, pattern: &[f64; 64]) -> Plane<Color16> {
    let mut result = Plane::new(image.width(), image.height());
    for (x, y, color) in image.enumerate_pixels() {
        let original_color = Color32::from(color);
        if original_color.a == 0 {
            result.set(x, y, Color16::TRANSPARENT);
        } else {
            let correction = get_wrapped(pattern, x, y);
            let corrected_color = Color32::new(
                ((original_color.r as f64 + correction * COLOR_RADIUS) as i32).clamp(0, 255),
                ((original_color.g as f64 + correction * COLOR_RADIUS) as i32).clamp(0, 255),
                ((original_color.b as f64 + correction * COLOR_RADIUS) as i32).clamp(0, 255),
                255,
            );
            result.set(x, y, Color16::from(corrected_color));
        }
    }
    result
}

pub fn convert_ordered4(image: &RgbaImage) -> Plane<Color16> {
    ordered_dithering(image, &BAYER_FLOAT_4X4)
}

pub fn convert_ordered8(image: &RgbaImage) -> Plane<Color16> {
    ordered_dithering(image, &BAYER_FLOAT_8X8)
}
