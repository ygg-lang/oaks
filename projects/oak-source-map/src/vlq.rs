//! VLQ Base64 encoding and decoding for Source Maps.
//!
//! VLQ (Variable Length Quantity) encoding is used in Source Map v3
//! to compactly represent integers in the mappings string.

use crate::{Result, SourceMapError};

const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

const BASE64_DECODE: [i8; 256] = {
    let mut table = [-1i8; 256];
    let mut i = 0;
    while i < 64 {
        table[BASE64_CHARS[i] as usize] = i as i8;
        i += 1;
    }
    table
};

const VLQ_CONTINUATION_BIT: u8 = 0b100000;
const VLQ_BASE_MASK: u8 = 0b011111;

/// Encodes a signed integer to VLQ Base64 string.
///
/// # Example
///
/// ```
/// use oak_source_map::vlq_encode;
///
/// assert_eq!(vlq_encode(0), "A");
/// assert_eq!(vlq_encode(1), "C");
/// assert_eq!(vlq_encode(-1), "D");
/// assert_eq!(vlq_encode(16), "gB");
/// assert_eq!(vlq_encode(12345), "uBtC");
/// ```
pub fn vlq_encode(value: i32) -> String {
    let mut result = String::new();
    let mut value = if value < 0 { ((-value) << 1) + 1 } else { value << 1 };

    loop {
        let mut digit = (value & VLQ_BASE_MASK as i32) as u8;
        value >>= 5;

        if value > 0 {
            digit |= VLQ_CONTINUATION_BIT;
        }

        result.push(BASE64_CHARS[digit as usize] as char);

        if value == 0 {
            break;
        }
    }

    result
}

/// Decodes a VLQ Base64 string to a signed integer.
///
/// # Example
///
/// ```
/// use oak_source_map::vlq_decode;
///
/// assert_eq!(vlq_decode("A"), Ok((0, 1)));
/// assert_eq!(vlq_decode("C"), Ok((1, 1)));
/// assert_eq!(vlq_decode("D"), Ok((-1, 1)));
/// assert_eq!(vlq_decode("gB"), Ok((16, 2)));
/// ```
pub fn vlq_decode(s: &str) -> Result<(i32, usize)> {
    vlq_decode_from_slice(s.as_bytes())
}

/// Decodes VLQ from a byte slice.
pub fn vlq_decode_from_slice(bytes: &[u8]) -> Result<(i32, usize)> {
    let mut result: i32 = 0;
    let mut shift: u32 = 0;
    let mut count = 0;

    for &byte in bytes {
        count += 1;

        let decoded = BASE64_DECODE[byte as usize];
        if decoded < 0 {
            return Err(SourceMapError::invalid_vlq(count - 1, format!("Invalid Base64 character: '{}'", byte as char)));
        }

        let decoded = decoded as u8;

        result |= ((decoded & VLQ_BASE_MASK) as i32) << shift;
        shift += 5;

        if decoded & VLQ_CONTINUATION_BIT == 0 {
            break;
        }
    }

    let is_negative = result & 1 != 0;
    result >>= 1;

    if is_negative {
        result = -result;
    }

    Ok((result, count))
}

/// Encodes multiple values into a VLQ string with separators.
pub fn vlq_encode_many(values: &[i32]) -> String {
    values.iter().map(|v| vlq_encode(*v)).collect()
}

/// Decodes multiple VLQ values from a string segment.
pub fn vlq_decode_many(s: &str) -> Result<Vec<i32>> {
    let bytes = s.as_bytes();
    let mut pos = 0;
    let mut result = Vec::new();

    while pos < bytes.len() {
        let (value, consumed) = vlq_decode_from_slice(&bytes[pos..])?;
        result.push(value);
        pos += consumed;
    }

    Ok(result)
}
