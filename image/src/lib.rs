pub mod canvas;
pub mod color;
mod utils;

#[cfg(test)]
mod test_pic {
    use canvas::Canvas;
    use color::RGBAColor;
    use indoc::indoc;

    use super::*;

    #[test]
    fn mutate_pixel() {
        let mut p = Canvas::with_size(15, 15);
        p.set_pixel_color((0, 0), RGBAColor::from((12, 12, 12)));
        p.set_pixel_color((10, 10), RGBAColor::from((12, 12, 12)));
        p.set_pixel_color((2, 10), RGBAColor::from((12, 12, 12)));

        assert_ne!(p.get_color_at((0, 0)), p.get_color_at((1, 1)));
    }

    #[test]
    fn basic_ppm() {
        // Check that the headers work right
        let p = Canvas::with_size(0, 0);
        assert_eq!("P3\n0 0\n255\n", p.to_ppm());

        // Check that the pixels are rendered correctly
        let p = Canvas::with_size(1, 1);
        assert_eq!("P3\n1 1\n255\n0 0 0\n", p.to_ppm());
    }

    #[test]
    fn larger_ppm() {
        // Check a simple, and empty picture
        let p = Canvas::with_size(3, 5);
        let expected = indoc! {"
            P3
            5 3
            255
            0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
            0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
        "};
        assert_eq!(expected, p.to_ppm());

        let mut p = Canvas::with_size(2, 11);
        for i in 0..p.height {
            for j in 0..p.width {
                p.set_pixel_color((i, j), RGBAColor::from((1.0, 0.8, 0.6)));
            }
        }

        let expected = indoc! {"
            P3
            11 2
            255
            255 205 154 255 205 154 255 205 154 255 205 154 255 205 154
            255 205 154 255 205 154 255 205 154 255 205 154 255 205 154
            255 205 154
            255 205 154 255 205 154 255 205 154 255 205 154 255 205 154
            255 205 154 255 205 154 255 205 154 255 205 154 255 205 154
            255 205 154
        "};
        assert_eq!(expected, p.to_ppm());
    }

    #[test]
    fn save_ppm() {
        let mut p = Canvas::with_size(1080, 1920);
        for col in 0..p.width {
            for row in 0..p.height {
                let red = (row as f64) / p.height as f64;
                let blue = (col as f64) / p.width as f64;
                p.set_pixel_color((row, col), RGBAColor::from((red, (red + blue) / 2.0, blue)));
            }
        }
        p.save_as_ppm("../test_outputs/grad.ppm".to_string());
    }
}
