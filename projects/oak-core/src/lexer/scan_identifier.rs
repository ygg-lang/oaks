/// Checks if the given byte is a valid start of an ASCII identifier ([a-zA-Z_]).
#[inline]
#[allow(dead_code)]
pub fn is_ascii_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

/// Checks if the given byte is a valid continuation of an ASCII identifier ([a-zA-Z0-9_]).
#[inline]
#[allow(dead_code)]
pub fn is_ascii_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}
