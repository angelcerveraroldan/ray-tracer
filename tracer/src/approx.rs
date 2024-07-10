const EPSILON: f64 = 0.0001;

/// Helper function to check equality of floating point integers
///
/// We say two real numbers are equal if the difference between them is
/// smaller than some epsilon constant epsilon
pub fn approx(x: f64, y: f64) -> bool {
    match (x - y).abs() < EPSILON {
        true => true,
        false => {
            {
                #[cfg(test)]
                println!(
                    "==== FLOAT APPROX ERROR: |{} - {}| = {} ≮ {} ====",
                    x,
                    y,
                    (x - y).abs(),
                    EPSILON
                );
            }

            false
        }
    }
}

#[cfg(test)]
mod approx_test {
    use crate::approx::approx;

    #[test]
    fn close_nums() {
        // Assert exact equality
        assert!(approx(1.12, 1.12));
        // Assert close numbers
        assert!(approx(1.12001, 1.12002));
        assert!(approx(12.12001, 12.12002));
        assert!(approx(-12.12001, -12.12002));
    }

    #[test]
    fn far_nums() {
        // Assert exact equality
        assert!(!approx(-1.12, 1.12));
        // Assert far numbers
        assert!(!approx(1.13001, 1.12002));
        assert!(!approx(12.13001, 12.12002));
        assert!(!approx(12.12001, -12.12002));
    }
}
