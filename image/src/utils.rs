pub(crate) fn rgb_percentage_to_u8(f: f64) -> Option<u8> {
    if !(0.0..=1.0).contains(&f) {
        return None;
    }

    Some((f * 256.0).round() as u8)
}

#[cfg(test)]
mod test_utils {
    use super::*;

    #[test]
    fn test_normalize_function() {
        let did_not_parse = None;
        assert_eq!(rgb_percentage_to_u8(1.0), Some(255));
        assert_eq!(rgb_percentage_to_u8(1.1), did_not_parse);
        assert_eq!(rgb_percentage_to_u8(-0.5), did_not_parse);
    }
}
