pub(crate) fn to_exitcode_u8(value: i32) -> u8 {
    if value == i32::MIN {
        return 255;
    }
    u8::try_from(value.abs()).unwrap_or(255)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_convert_to_u8() {
        assert_eq!(to_exitcode_u8(0), 0);
        assert_eq!(to_exitcode_u8(1), 1);
        assert_eq!(to_exitcode_u8(-1), 1);
        assert_eq!(to_exitcode_u8(255), 255);
        assert_eq!(to_exitcode_u8(-255), 255);
        assert_eq!(to_exitcode_u8(256), 255);
        assert_eq!(to_exitcode_u8(-256), 255);
        assert_eq!(to_exitcode_u8(i32::MAX), 255);
        assert_eq!(to_exitcode_u8(i32::MIN), 255);
    }
}
