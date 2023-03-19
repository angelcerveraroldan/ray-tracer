// Check if two floats are equal to within epsilon decimals
pub fn float_eq(f: f64, s: f64, epsilon: u16) -> bool {
    let mult = 10.0_f64.powf(epsilon as f64);
    let f_int = (f * mult) as i64;
    let s_int = (s * mult) as i64;

    f_int == s_int
}