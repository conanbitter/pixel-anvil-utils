use std::ops;

use image::{Rgb, Rgba};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color16(pub u16);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Color32 {
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub a: i32,
}

const fn get_color_map() -> [i32; 32] {
    let mut result = [0; 32];
    let mut i = 0;
    while i < 32 {
        result[i] = ((i as f64) * 255.0 / 31.0) as i32;
        i += 1;
    }
    result
}

const COLORS_5TO8: [i32; 32] = get_color_map();

impl Color16 {
    pub const TRANSPARENT: Color16 = Color16(0);
}

impl Default for Color16 {
    fn default() -> Self {
        Self(0b1000000000000000)
    }
}

impl Color32 {
    pub fn new(r: i32, g: i32, b: i32, a: i32) -> Color32 {
        Color32 { r, g, b, a }
    }

    pub fn distance_squared(color1: Color32, color2: Color32) -> i32 {
        let r = color1.r - color2.r;
        let g = color1.g - color2.g;
        let b = color1.b - color2.b;
        r * r + g * g + b * b
    }
}

impl Default for Color32 {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

impl ops::Add<Color32> for Color32 {
    type Output = Color32;

    fn add(self, rhs: Color32) -> Self::Output {
        Color32 {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: 255,
        }
    }
}

impl ops::AddAssign<Color32> for Color32 {
    fn add_assign(&mut self, rhs: Color32) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl ops::Sub<Color32> for Color32 {
    type Output = Color32;

    fn sub(self, rhs: Color32) -> Self::Output {
        Color32 {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
            a: 255,
        }
    }
}

impl ops::Mul<i32> for Color32 {
    type Output = Color32;

    fn mul(self, rhs: i32) -> Self::Output {
        Color32 {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: 255,
        }
    }
}

impl From<Color32> for Color16 {
    fn from(color: Color32) -> Self {
        let r = (color.r >> 3).clamp(0, 31) as u16;
        let g = (color.g >> 3).clamp(0, 31) as u16;
        let b = (color.b >> 3).clamp(0, 31) as u16;
        let a = color.a.clamp(0, 1) as u16;
        let result = a << 15 | r << 10 | g << 5 | b;
        Color16(result)
    }
}

impl From<Color16> for Color32 {
    fn from(color: Color16) -> Self {
        let r = ((color.0 >> 10) & 0b11111) as usize;
        let g = ((color.0 >> 5) & 0b11111) as usize;
        let b = (color.0 & 0b11111) as usize;
        Color32 {
            r: COLORS_5TO8[r],
            g: COLORS_5TO8[g],
            b: COLORS_5TO8[b],
            a: if (color.0 >> 15) > 0 { 255 } else { 0 },
        }
    }
}

impl From<&Rgb<u8>> for Color32 {
    fn from(value: &Rgb<u8>) -> Self {
        Color32 {
            r: value[0] as i32,
            g: value[1] as i32,
            b: value[2] as i32,
            a: 255,
        }
    }
}

impl From<&Rgba<u8>> for Color32 {
    fn from(value: &Rgba<u8>) -> Self {
        if value[3] < 128 {
            Color32 { r: 0, g: 0, b: 0, a: 0 }
        } else {
            Color32 {
                r: value[0] as i32,
                g: value[1] as i32,
                b: value[2] as i32,
                a: 255,
            }
        }
    }
}
