use image::{RgbImage, RgbaImage};

use crate::{
    color::{Color16, Color32},
    plane::Plane,
};

// region: posterize

pub fn convert_posterize(image: &RgbImage) -> Plane<Color16> {
    let mut result = Plane::new(image.width(), image.height());

    for (x, y, color) in image.enumerate_pixels() {
        result.set(x, y, Color16::from(Color32::from(color)));
    }

    result
}

// endregion
