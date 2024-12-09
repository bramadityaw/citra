use crate::citra::image;

pub enum Color {
    White,
    Black,
    Red,
    Green,
    Blue,
}

impl image::AsPixel for Color {
    fn as_pixel(&self) -> image::Pixel {
        match *self {
            Color::Black => image::Pixel::default(),
            Color::White => image::Pixel {r: 255, g: 255, b: 255},
            Color::Red   => image::Pixel {r: 255, g: 0, b: 0},
            Color::Green => image::Pixel {r: 0, g: 255, b: 0},
            Color::Blue  => image::Pixel {r: 0, g: 0, b: 255},
        }
    }
}


