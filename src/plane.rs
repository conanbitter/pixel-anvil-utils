use std::{clone::Clone, fs, io::Write, ops::AddAssign, path::Path};

use crate::color::Color16;

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
    pub fn save<P: AsRef<Path>>(&self, filename: P) -> anyhow::Result<()> {
        let mut file = fs::File::create(filename)?;

        // RLE_ZERO compression
        let mut data: Vec<u8> = vec![];
        let mut accum = 0u8;
        for pixel in &self.data {
            if pixel.is_transparent() {
                if accum < 128 {
                    accum += 1;
                } else {
                    data.push(accum - 1);
                    accum = 0;
                }
            } else {
                if accum > 0 {
                    data.push(accum - 1);
                    accum = 0;
                }
                let bytes: [u8; 2] = pixel.0.to_le_bytes();
                data.push(bytes[0]);
                data.push(bytes[1]);
            }
        }

        // Magic
        file.write_all("IMG ".as_bytes())?;

        // Size
        let size: u32 = data.len() as u32 + 2 * 4; // data+width+height
        file.write_all(&size.to_le_bytes())?;

        // Measurements
        file.write_all(&self.width.to_le_bytes())?;
        file.write_all(&self.height.to_le_bytes())?;

        // Data
        file.write_all(&data)?;

        Ok(())
    }
}
