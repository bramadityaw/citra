use std::io::{self, Write};
use std::default::Default;
use std::fs::File;

use crate::citra::filter::{Filter, FilterFunc};
use crate::citra::filter;
use crate::citra::draw::{Draw, DrawErr};

pub type Point = (usize, usize);

#[derive(Default, Copy, Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub trait AsPixel {
    fn as_pixel(&self) -> Pixel;
}

impl AsPixel for Pixel {
    fn as_pixel(&self) -> Pixel {
        *self
    }
}

pub trait Image {
    fn new(w: usize, h: usize) -> Self;
    fn dims(&self) -> (usize, usize);

    fn size(&self) -> usize {
        let (w, h) = self.dims();
        w * h
    }
}

#[derive(Clone)]
pub enum BinVal {
    Object,
    Backgr,
}

impl AsPixel for BinVal {
    fn as_pixel(&self) -> Pixel {
        use crate::citra::color::Color;
        match *self {
            BinVal::Object => Color::Black.as_pixel(),
            BinVal::Backgr => Color::White.as_pixel(),
        }
    }
}

pub struct BinImage {
    w: usize,
    h: usize,
    data: Vec<BinVal>,
}

impl Image for BinImage {
    fn new(w: usize, h: usize) -> Self {
        BinImage {
            w,
            h,
            data: vec![BinVal::Backgr; w*h],
        }
    }

    fn dims(&self) -> (usize, usize) {
        (self.w, self.h)
    }
}

#[derive(Default, Clone)]
pub struct RGBImage {
    w: usize,
    h: usize,
    depth: usize,
    data: Vec<Pixel>,
}

impl Image for RGBImage {
    fn new(w: usize, h: usize) -> Self {
        RGBImage {
            w,
            h,
            depth: u8::MAX as usize,
            data: vec![Pixel::default(); w*h],
        }
    }

    fn dims(&self) -> (usize, usize) {
        (self.w, self.h)
    }
}

impl Filter for RGBImage {
    fn filter(self, f: FilterFunc) -> Self {
        let mut img = Self::new(self.w, self.h);
        for i in 0..self.data.len() {
            img.data[i] = f(self.data[i]);
        }
        img
    }
}

impl RGBImage {
    pub fn into_binary(&self, thresh: u8) -> BinImage {
        let mut bin = BinImage::new(self.w, self.h);
        let ori = self.to_grayscale();
        for i in 0..self.data.len() {
            let px = ori.data[i].r;
            bin.data[i] = if px >= thresh {
                BinVal::Object
            } else {
                BinVal::Backgr
            };
        }

        bin
    }

    pub fn to_grayscale(&self) -> Self {
        self.clone().filter(filter::grayscale)
    }

    pub fn fill(&mut self, color: impl AsPixel) {
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
}

impl Draw for RGBImage {
    fn draw_dot(&mut self, point: Point, pixel: Pixel) -> Result<(), DrawErr> {
        let (x, y) = point;
        if x > self.w || y > self.h {
            eprintln!("Point out of bounds.\nImage width: {}\nImage height: {}\nAttempt to draw point: {} {}", self.w, self.h, x, y);
            return Err(DrawErr::OutOfBounds);
        }
        self.data[x + y * self.w] = pixel;
        Ok(())
    }
}
