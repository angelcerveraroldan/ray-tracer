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

    pub fn mutate_pixel() {}

    pub fn to_ppi(&self) -> String {
        fn row_to_ppi(row: &Vec<Color>) -> String {
            let mut s = String::new();
            row.iter().for_each(|pixel| {
                let (r, g, b) = pixel.to_rgb().unwrap();
                let mut pixel_rgb = format!("{} {} {} ", r, g, b,);
                if s.len() + pixel_rgb.len() > 70 {
                    pixel_rgb = format!("\n{}", pixel_rgb);
                }
                s = format!("{}{}", s, pixel_rgb);
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
    fn basic_ppi() {
        // Check that the headers work right
        let p = Picture::new(0, 0);
        assert_eq!("P3\n0 0\n255\n", p.to_ppi());

        // Check that the pixels are rendered correctly
        let p = Picture::new(1, 1);
        assert_eq!("P3\n1 1\n255\n0 0 0\n", p.to_ppi());
    }

    #[test]
    fn larger_ppi() {
        let p = Picture::new(3, 5);
        let expected = "P3\n5 3\n255\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n";
        assert_eq!(expected, p.to_ppi());
    }
}
