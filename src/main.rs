use std::io;

use crate::citra::image::*;

pub mod citra;

fn main() -> io::Result<()> {
    let mut img = Image::new(600, 800, 255);
    img.fill(Color::White);
    //img.draw_dot_color((1, 1), Color::Black)?;
    //img.draw_line_color((10, 10), (10, 50), Color::Black)?;
    //img.draw_line_color((10, 10), (50, 10), Color::Black)?;
    img.draw_line_color((10, 50), (50, 10), Color::Black)?;
    img.save("test")?;
    Ok(())
}
