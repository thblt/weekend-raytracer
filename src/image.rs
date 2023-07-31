use crate::matrix::Matrix;
use crate::vec3::Color;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

pub type Image = Matrix<Color>;

impl Image {
    /// Write this Matrix as a PPM image path.
    pub fn write_ppm(self, path: &str) -> std::io::Result<()> {
        let mut writer = BufWriter::new(File::create(path)?);
        write!(writer, "P3\n{} {}\n255\n", self.width(), self.height())?;
        let mut row = 0;
        for pixel in &self.vec {
            let red = (255.0 * pixel.x).round() as u8;
            let green = (255.0 * pixel.y).round() as u8;
            let blue = (255.0 * pixel.z).round() as u8;
            write!(writer, "{} {} {}", red, green, blue)?;
            row += 1;
            if row == self.width() {
                write!(writer, "\n")?;
                row = 0;
            } else {
                write!(writer, " ")?;
            }
        }
        Ok(())
    }
}
