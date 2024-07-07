use crate::errors::ImageErrors;

pub(crate) fn rgb_percentage_to_u8(f: f64) -> Result<u8, ImageErrors> {
    if f > 1.0 || f < 0.0 {
        return Err(ImageErrors::ColorPercentageParseError);
    }

    Ok((f * 255.0).ceil() as u8)
}

#[cfg(test)]
mod test_utils {
    use super::*;

    #[test]
    fn test_normalize_function() {
        let did_not_parse = Err(ImageErrors::ColorPercentageParseError);
        assert_eq!(rgb_percentage_to_u8(1.0), Ok(255));
        assert_eq!(rgb_percentage_to_u8(1.1), did_not_parse);
        assert_eq!(rgb_percentage_to_u8(-0.5), did_not_parse);
    }
}
