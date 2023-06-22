use crate::source::SimdScanner;

/// Counts how many ASCII digits are at the start of the byte slice.
#[inline]
pub fn count_ascii_digit_prefix(bytes: &[u8]) -> usize {
    SimdScanner::skip_ascii_digits(bytes)
}

/// Counts how many ASCII hexadecimal digits are at the start of the byte slice.
#[inline]
pub fn count_ascii_hexdigit_prefix(bytes: &[u8]) -> usize {
    SimdScanner::skip_ascii_hexdigits(bytes)
}
