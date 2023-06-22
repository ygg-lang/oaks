/// Checks if a byte can start an ASCII identifier (a-z, A-Z, or _).
#[inline]
pub fn is_ascii_ident_start(b: u8) -> bool {
    b == b'_' || b.is_ascii_alphabetic()
}

/// Checks if a byte can continue an ASCII identifier (a-z, A-Z, 0-9, or _).
#[inline]
pub fn is_ascii_ident_continue(b: u8) -> bool {
    is_ascii_ident_start(b) || b.is_ascii_digit()
}
