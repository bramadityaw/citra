use crate::citra::image::{Pixel, Image};

pub trait Filter: Image {
    fn filter(self, f: FilterFunc) -> Self;
}

pub type FilterFunc = fn (Pixel) -> Pixel;

pub fn grayscale(px: Pixel) -> Pixel {
    let y = (0.299 * px.r as f32 +
             0.587 * px.g as f32 +
             0.144 * px.b as f32).floor() as u8;
    Pixel {
        r: y,
        g: y,
        b: y,
    }
}
