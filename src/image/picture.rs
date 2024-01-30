use std::{fs, io::Write, ops::Add};

use super::color::Color;

struct Picture {
    height: usize,
    width: usize,
    pixels: Vec<Vec<Color>>,
}

impl Picture {
    pub fn new(height: usize, width: usize) -> Self {
        let row = vec![Color::default(); width];
        let pixels = vec![row; height];
        Self {
            height,
            width,
            pixels,
        }
    }

    pub fn save_as_ppm(&self, to: String) {
        let file_content = self.to_ppm();
        fs::File::create(to)
            .unwrap()
            .write(file_content.as_bytes())
            .unwrap();
    }

    // TODO: Add better error handling  (not using string)
    pub fn mutate_pixel(&mut self, row: usize, col: usize, new_color: Color) -> Result<(), String> {
        *self
            .pixels
            .get_mut(row)
            .ok_or("Row not in bound")?
            .get_mut(col)
            .ok_or("Col not in bound")? = new_color;

        Ok(())
    }

    pub fn to_ppm(&self) -> String {
        fn row_to_ppi(row: &Vec<Color>) -> String {
            let mut s = String::new();
            let mut char_count = 0;

            row.iter().for_each(|pixel| {
                let (r, g, b) = pixel.to_rgb().unwrap();
                let mut pixel_rgb = format!("{} {} {} ", r, g, b,);

                // Some PPM readers only accept files with a max line length of 70
                if char_count + pixel_rgb.len() > 70 {
                    // Remove ending space
                    s.pop();
                    s.push('\n');
                    char_count = 0;
                }
                s = format!("{}{}", s, pixel_rgb);
                char_count += pixel_rgb.len();
            });

            // Remove last space in the line
            s.pop();
            format!("{}\n", s)
        }

        let ppi_header = format!("P3\n{} {}\n255\n", self.width, self.height);
        let pixels_strings: Vec<String> = self.pixels.iter().map(row_to_ppi).collect();
        format!("{}{}", ppi_header, pixels_strings.join(""))
    }
}

#[cfg(test)]
mod test_pic {
    use super::*;

    #[test]
    fn mutate_pixel() {
        let mut p = Picture::new(10, 10);
        let good = p.mutate_pixel(0, 0, Color::new(12.0, 12.0, 12.0));
        let bad = p.mutate_pixel(10, 10, Color::new(12.0, 12.0, 12.0));
        let bad1 = p.mutate_pixel(2, 10, Color::new(12.0, 12.0, 12.0));
        assert_eq!(good, Ok(()));
        assert_eq!(bad, Err("Row not in bound".to_string()));
        assert_eq!(bad1, Err("Col not in bound".to_string()));
    }

    #[test]
    fn basic_ppm() {
        // Check that the headers work right
        let p = Picture::new(0, 0);
        assert_eq!("P3\n0 0\n255\n", p.to_ppm());

        // Check that the pixels are rendered correctly
        let p = Picture::new(1, 1);
        assert_eq!("P3\n1 1\n255\n0 0 0\n", p.to_ppm());
    }

    #[test]
    fn larger_ppm() {
        /// Check a simple, and empty picture
        let p = Picture::new(3, 5);
        let expected = r#"P3
5 3
255
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
"#;
        assert_eq!(expected, p.to_ppm());

        let mut p = Picture::new(2, 10);
        for i in (0..p.height) {
            for j in (0..p.width) {
                p.mutate_pixel(i, j, Color::new(1.0, 0.8, 0.6)).unwrap();
            }
        }

        println!("{}", p.to_ppm());

        let expected = r#"P3
10 2
255
255 205 154 255 205 154 255 205 154 255 205 154 255 205 154
255 205 154 255 205 154 255 205 154 255 205 154 255 205 154
255 205 154 255 205 154 255 205 154 255 205 154 255 205 154
255 205 154 255 205 154 255 205 154 255 205 154 255 205 154
"#;
        assert_eq!(expected, p.to_ppm());
    }

    #[test]
    fn save_ppm() {
        let mut p = Picture::new(1080, 1920);
        for col in (0..p.width) {
            for row in (0..p.height) {
                let red = (row as f64) / p.height as f64;
                let blue = (col as f64) / p.width as f64;
                p.mutate_pixel(row, col, Color::new(red, (red + blue) / 2.0, blue));
            }
        }
        p.save_as_ppm("./src/test/xxx.ppm".to_string());
    }
}
