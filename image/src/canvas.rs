use std::io::Write;

use crate::color::{self, RGBAColor};

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,

    pixels: Vec<Vec<color::RGBAColor>>,
}

impl Canvas {
    pub fn with_size(height: usize, width: usize) -> Self {
        let row = vec![color::RGBAColor::default(); width];
        let pixels = vec![row; height];

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn background<A>(self, color: A) -> Self
    where
        RGBAColor: From<A>,
    {
        let row = vec![color.into(); self.width];
        let pixels = vec![row; self.height];

        Self {
            width: self.width,
            height: self.height,
            pixels,
        }
    }

    pub fn set_pixel_color<A>(&mut self, (row, col): (usize, usize), color: A) -> &mut Self
    where
        RGBAColor: From<A>,
    {
        self.pixels[row][col] = color.into();
        self
    }

    pub fn get_color_at(&self, (row, col): (usize, usize)) -> Option<&RGBAColor> {
        self.pixels.get(row).and_then(|r| r.get(col))
    }
}

////////////////////////////////////////////////////////
//                       PPM                          //
////////////////////////////////////////////////////////

impl Canvas {
    pub fn to_ppm(&self) -> String {
        fn row_to_ppi(row: &Vec<color::RGBAColor>) -> String {
            let mut s = String::new();
            let mut char_count = 0;

            row.iter().for_each(|pixel| {
                let pixel_rgb = format!("{} {} {} ", pixel.red, pixel.green, pixel.blue);

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

        let (height, width) = (self.height, self.width);
        let ppi_header = format!("P3\n{} {}\n255\n", width, height);
        let pixels_strings: Vec<String> = self.pixels.iter().map(row_to_ppi).collect();
        format!("{}{}", ppi_header, pixels_strings.join(""))
    }

    pub fn save_as_ppm(&self, to: String) {
        let file_content = self.to_ppm();
        let mut file = std::fs::File::create(to).unwrap();
        let _ = file.write_all(file_content.as_bytes());
    }
}
