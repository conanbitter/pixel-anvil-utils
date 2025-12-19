use std::path::Path;

use image::{ImageReader, RgbaImage};

use crate::{
    color::Color16,
    converters::{convert_ordered4, convert_posterize},
    plane::Plane,
};

mod color;
mod converters;
mod plane;

fn image_convert<P: AsRef<Path>>(
    input: P,
    output: P,
    dithering: fn(&RgbaImage) -> Plane<Color16>,
) -> anyhow::Result<()> {
    let img = ImageReader::open(input)?.decode()?.to_rgba8();
    let res_img = dithering(&img);
    res_img.save(output)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    image_convert("assets/rainbow.png", "assets/rainbow.img", convert_ordered4)?;

    Ok(())
}
