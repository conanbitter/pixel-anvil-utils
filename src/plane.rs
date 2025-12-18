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

        // Magic
        file.write_all("IMG ".as_bytes())?;

        // Size
        let size = self.width * self.height * 2 + 2 * 4; // data+width+height
        file.write_all(&size.to_le_bytes())?;

        // Measurements
        file.write_all(&self.width.to_le_bytes())?;
        file.write_all(&self.height.to_le_bytes())?;

        // Data
        for pixel in &self.data {
            file.write_all(&(pixel.0).to_le_bytes())?;
        }

        Ok(())
    }
}
