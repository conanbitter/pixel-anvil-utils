use std::{clone::Clone, fs, io::Write, ops::AddAssign, path::Path};

use image::RgbaImage;

use crate::color::{Color16, Color32};

pub struct Plane<T> {
    data: Vec<T>,
    pub width: u32,
    pub height: u32,
}

impl<T: Default + Clone + Copy> Plane<T> {
    pub fn new(width: u32, height: u32) -> Plane<T> {
        Plane {
            width,
            height,
            data: vec![T::default(); (width * height) as usize],
        }
    }

    pub fn set(&mut self, x: u32, y: u32, value: T) {
        self.data[(x + y * self.width) as usize] = value;
    }

    pub fn get(&self, x: u32, y: u32) -> T {
        self.data[(x + y * self.width) as usize]
    }
}

impl<T: AddAssign> Plane<T> {
    pub fn add(&mut self, x: u32, y: u32, value: T) {
        self.data[(x + y * self.width) as usize] += value;
    }
}

impl Plane<Color16> {
    const MAX_TRANSP_COUNT: u16 = 1 << 15;

    pub fn save<P: AsRef<Path>>(&self, filename: P) -> anyhow::Result<()> {
        let mut file = fs::File::create(filename)?;

        // RLE_ZERO compression
        let mut data: Vec<u16> = vec![];
        let mut accum = 0u16;
        for pixel in &self.data {
            if pixel.is_transparent() {
                if accum < Plane::MAX_TRANSP_COUNT {
                    accum += 1;
                } else {
                    data.push(accum - 1);
                    accum = 1;
                }
            } else {
                if accum > 0 {
                    data.push(accum - 1);
                    accum = 0;
                }
                data.push(pixel.0);
            }
        }
        if accum > 0 {
            data.push(accum - 1);
        }

        // Magic
        file.write_all("IMG ".as_bytes())?;

        // Size
        let size: u32 = data.len() as u32 + 2 * 4; // data+width+height
        //let size: u32 = self.width * self.height * 2 + 2 * 4;
        file.write_all(&size.to_le_bytes())?;

        // Measurements
        file.write_all(&self.width.to_le_bytes())?;
        file.write_all(&self.height.to_le_bytes())?;

        // Data
        for entry in data {
            file.write_all(&entry.to_le_bytes())?;
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn debud_save<P: AsRef<Path>>(&self, filename: P) -> anyhow::Result<()> {
        let mut img = RgbaImage::new(self.width, self.height);
        for (x, y, color) in img.enumerate_pixels_mut() {
            let src_color = Color32::from(self.get(x, y));
            color[0] = src_color.r as u8;
            color[1] = src_color.g as u8;
            color[2] = src_color.b as u8;
            color[3] = src_color.a as u8;
        }
        img.save(filename)?;
        Ok(())
    }
}
