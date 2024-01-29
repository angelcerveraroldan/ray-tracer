use crate::image::RGBPercentage;

pub struct Canvas {
    height: usize,
    width: usize,
    canvas: Vec<Vec<RGBPercentage>>,
}

impl Canvas {
    pub fn new(height: usize, width: usize) -> Canvas {
        let empty_row: Vec<RGBPercentage> = (0..width).map(|_| RGBPercentage::default()).collect();
        let canvas = (0..height).map(|_| empty_row.clone()).collect();

        Canvas {
            height,
            width,
            canvas,
        }
    }

    pub fn mutate_pixel(
        &mut self,
        row: usize,
        col: usize,
        pixel: RGBPercentage,
    ) -> Option<(usize, usize)> {
        *self.canvas.get_mut(row)?.get_mut(col)? = pixel;
        Some((row, col))
    }
}

#[cfg(test)]
mod test_canvas {
    use super::*;

    #[test]
    fn empty_canvas() {
        let canvas = Canvas::new(10, 20);

        assert_eq!(canvas.height, 10);
        assert_eq!(canvas.width, 20);

        for row in canvas.canvas {
            for pixel in row {
                assert_eq!(pixel, RGBPercentage::new(0.0, 0.0, 0.0))
            }
        }
    }

    #[test]
    fn modified_canvas() {
        let mut canvas = Canvas::new(10, 20);
        canvas.mutate_pixel(0, 9, RGBPercentage::new(1.0, 1.0, 1.0));
        canvas.mutate_pixel(9, 19, RGBPercentage::new(1.0, 1.0, 1.0));
        assert_eq!(canvas.canvas[0][9], RGBPercentage::new(1.0, 1.0, 1.0));
        assert_eq!(canvas.canvas[9][19], RGBPercentage::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn out_of_bounds() {
        let mut canvas = Canvas::new(10, 20);
        let mutated = canvas.mutate_pixel(100, 100, RGBPercentage::new(0.0, 12.0, 23.1));
        assert_eq!(mutated, None)
    }
}
