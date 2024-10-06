use std::io::{self, Write};
use std::default::Default;
use std::fs::File;

#[derive(Default, Clone)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

pub enum Color {
    White,
    Black,
    Red,
    Green,
    Blue,
}

impl Color {
    pub fn as_pixel(&self) -> Pixel {
        match *self {
            Color::Black => Default::default(),
            Color::White => Pixel {r: 255, g: 255, b: 255},
            Color::Red   => Pixel {r: 255, g: 0, b: 0},
            Color::Green => Pixel {r: 0, g: 255, b: 0},
            Color::Blue  => Pixel {r: 0, g: 0, b: 255},
        }
    }
}

pub enum ImageError<E> {
    EditError(E),
}

impl From<ImageError<String>> for io::Error {
    fn from(img: ImageError<String>) -> io::Error {
        use io::Error;
        match img {
            ImageError::EditError(msg) => Error::other(msg),
        }
    }
}

pub struct Image {
    w: usize,
    h: usize,
    depth: usize,
    data: Vec<Pixel>,
}

pub type Point = (usize, usize);

impl Image {
    pub fn new(w: usize, h: usize, depth: usize) -> Self {
        Image {
            w: w,
            h: h,
            depth: depth,
            data: vec![Default::default(); w*h],
        }
    }

    pub fn fill(&mut self, color: Color) {
        self.data.fill(color.as_pixel());
    }

    pub fn save(&self, name: &str) -> io::Result<()> {
        let filename = format!("{}.ppm", name);
        let mut f = File::create(filename)?;
        let header = self.header();
        f.write_all(header.as_bytes())?;
        for px in &self.data {
            let buf = [px.r, px.g, px.b];
            f.write_all(&buf)?;
        }
        Ok(())
    }
    
    fn header(&self) -> String {
        format!("P6 {} {} {} ", self.w, self.h, self.depth)
    }

    pub fn draw_dot_color(&mut self, point: Point, color: Color) -> Result<(), ImageError<String>> {
        self.draw_dot(point, color.as_pixel())?;
        Ok(())
    }

    pub fn draw_dot(&mut self, point: Point, pixel: Pixel) -> Result<(), ImageError<String>> {
        let (x, y) = point;
        if x > self.w || y > self.h {
            let msg = format!("Point out of bounds.\nImage width: {}\nImage height: {}\nAttempt to draw point: {} {}\n", self.w, self.h, x, y);
            return Err(ImageError::EditError(msg));
        }
        self.data[x + y * self.w] = pixel;
        Ok(())
    }

    pub fn draw_line_color(&mut self, from: Point, to: Point, color: Color) -> Result<(), ImageError<String>> {
        self.draw_line(from, to, color.as_pixel())?;
        Ok(())
    }

    pub fn draw_line(&mut self, from: Point, to: Point, pixel: Pixel) -> Result<(), ImageError<String>> {
        let (x1, y1) = from;
        let (x2, y2) = to;
        if x1 > self.w || x2 > self.w || y1 > self.h || y2 > self.h {
            let msg = format!("Point out of bounds.\nImage width: {}\nImage height: {}\nAttempt to draw these points:\n\tfrom: {} {}\n\tto: {} {}\n", self.w, self.h, x1, y1, x2, y2);
            return Err(ImageError::EditError(msg));
        }
        let (dx, dy) = (x2.abs_diff(x1), y2.abs_diff(y1));
        if dx == 0 {
        // Straight vertical line
            let x = x1;
            for y in y1..y2 {
                self.data[x + y * self.w] = pixel.clone();
            }
            return Ok(());
        }
        if dy == 0 {
        // Straight horizontal line
            let y = y1;
            for x in x1..x2 {
                self.data[x + y * self.w] = pixel.clone();
            }
            return Ok(());
        }
        // Diagonal line
        todo!("Diagonal line drawing");
        let mut e : f32 = 0.0;
        let de = (dy/dx) as f32;
        let mut y = 0;
        for x in x1..x2 {
            //draw it
            e = e + de;
            if f32::abs(e) >= 0.5 {
                y = y + 1;
                e = e - 1.0;
            }
        }
        Ok(())
    }
}
