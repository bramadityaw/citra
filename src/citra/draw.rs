use crate::citra::image::{Image, Point, Pixel, AsPixel};
use crate::citra::color;

pub enum DrawErr {
    OutOfBounds,
}

pub trait Draw: Image {
    fn draw_dot(&mut self, point: Point, pixel: Pixel) -> Result<(), DrawErr>;

    fn draw_dot_color(&mut self, point: Point, color: color::Color) -> Result<(), DrawErr> {
        self.draw_dot(point, color.as_pixel())?;
        Ok(())
    }

    fn draw_line(&mut self, from: Point, to: Point, pixel: Pixel) -> Result<(), DrawErr> {
        let (w, h) = self.dims();
        let (x1, y1) = from;
        let (x2, y2) = to;
        if x1 > w || x2 > w || y1 > h || y2 > h {
            println!("Point out of bounds!\nImage width: {}\nImage height: {}\nAttempt to draw these points:\n\tfrom: {} {}\n\tto: {} {}\n", w, h, x1, y1, x2, y2);
            return Err(DrawErr::OutOfBounds);
        }
        let (dx, dy) = (x2.abs_diff(x1), y2.abs_diff(y1));
        if dx == 0 {
        // Straight vertical line
            let x = x1;
            for y in y1..y2 {
                let point = (x, y);
                self.draw_dot(point, pixel)?;
            }
            return Ok(());
        }
        if dy == 0 {
        // Straight horizontal line
            let y = y1;
            for x in x1..x2 {
                let point = (x, y);
                self.draw_dot(point, pixel)?;
            }
            return Ok(());
        }
        // Diagonal line
        let mut d : isize = (2*dy - dx).try_into().unwrap();
        let mut y = y1;
        for x in x1..x2 {
            let point = (x, y);
            self.draw_dot(point, pixel)?;
            if d > 0 {
                y = y + 1;
                d = d - (2 * dx) as isize;
            }
            d = d + (2 * dy) as isize;
        }
        Ok(())
    }

    fn draw_line_color(&mut self, from: Point, to: Point, color: color::Color) -> Result<(), DrawErr> {
        self.draw_line(from, to, color.as_pixel())?;
        Ok(())
    }
}
