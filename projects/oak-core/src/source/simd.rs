use std::simd::prelude::*;

/// SIMD-accelerated scanning utilities.
pub struct SimdScanner;

impl SimdScanner {
    /// Finds the first occurrence of `needle` in `text`.
    #[inline(always)]
    pub fn find_byte(text: &[u8], needle: u8) -> Option<usize> {
        let mut i = 0;
        let len = text.len();
        const LANES: usize = 32;

        // Process 32 bytes at a time
        while i + LANES <= len {
            // SAFETY: We checked bounds.
            let chunk = Simd::<u8, LANES>::from_slice(unsafe { text.get_unchecked(i..i + LANES) });
            let mask = chunk.simd_eq(Simd::splat(needle));
            if let Some(idx) = mask.first_set() {
                return Some(i + idx);
            }
            i += LANES
        }

        // Process remaining bytes
        while i < len {
            if unsafe { *text.get_unchecked(i) } == needle {
                return Some(i);
            }
            i += 1
        }
        None
    }

    /// Skips bytes while they match `byte`. Returns number of skipped bytes.
    #[inline(always)]
    pub fn skip_byte(text: &[u8], byte: u8) -> usize {
        let mut i = 0;
        let len = text.len();
        const LANES: usize = 32;

        while i + LANES <= len {
            let chunk = Simd::<u8, LANES>::from_slice(unsafe { text.get_unchecked(i..i + LANES) });
            let mask = chunk.simd_eq(Simd::splat(byte));
            // If not all match, we found a stopper
            if !mask.all() {
                // !mask has 1s where characters DO NOT match
                let not_mask = !mask;
                if let Some(idx) = not_mask.first_set() {
                    return i + idx;
                }
            }
            i += LANES
        }

        while i < len {
            if unsafe { *text.get_unchecked(i) } != byte {
                break;
            }
            i += 1
        }
        i
    }

    /// Skips bytes while they match either of two bytes.
    #[inline(always)]
    pub fn skip_two_bytes(text: &[u8], b1: u8, b2: u8) -> usize {
        let mut i = 0;
        let len = text.len();
        const LANES: usize = 32;

        while i + LANES <= len {
            let chunk = Simd::<u8, LANES>::from_slice(unsafe { text.get_unchecked(i..i + LANES) });
            let m1 = chunk.simd_eq(Simd::splat(b1));
            let m2 = chunk.simd_eq(Simd::splat(b2));
            let mask = m1 | m2;
            if !mask.all() {
                let not_mask = !mask;
                if let Some(idx) = not_mask.first_set() {
                    return i + idx;
                }
            }
            i += LANES
        }

        while i < len {
            let b = unsafe { *text.get_unchecked(i) };
            if b != b1 && b != b2 {
                break;
            }
            i += 1
        }
        i
    }

    /// Skips common ASCII whitespace (' ', '\t', '\n', '\r').
    #[inline(always)]
    pub fn skip_ascii_whitespace(text: &[u8]) -> usize {
        let mut i = 0;
        let len = text.len();
        const LANES: usize = 32;
        while i + LANES <= len {
            let chunk = Simd::<u8, LANES>::from_slice(unsafe { text.get_unchecked(i..i + LANES) });
            let m1 = chunk.simd_eq(Simd::splat(b' '));
            let m2 = chunk.simd_eq(Simd::splat(b'\t'));
            let m3 = chunk.simd_eq(Simd::splat(b'\n'));
            let m4 = chunk.simd_eq(Simd::splat(b'\r'));
            let mask = m1 | m2 | m3 | m4;
            if !mask.all() {
                let not_mask = !mask;
                if let Some(idx) = not_mask.first_set() {
                    return i + idx;
                }
            }
            i += LANES
        }
        while i < len {
            let b = unsafe { *text.get_unchecked(i) };
            if b != b' ' && b != b'\t' && b != b'\n' && b != b'\r' {
                break;
            }
            i += 1
        }
        i
    }

    /// Skips ASCII digits ('0'-'9').
    #[inline(always)]
    pub fn skip_ascii_digits(text: &[u8]) -> usize {
        let mut i = 0;
        let len = text.len();
        const LANES: usize = 32;
        while i + LANES <= len {
            let chunk = Simd::<u8, LANES>::from_slice(unsafe { text.get_unchecked(i..i + LANES) });
            let ge0 = chunk.simd_ge(Simd::splat(b'0'));
            let le9 = chunk.simd_le(Simd::splat(b'9'));
            let mask = ge0 & le9;
            if !mask.all() {
                let not_mask = !mask;
                if let Some(idx) = not_mask.first_set() {
                    return i + idx;
                }
            }
            i += LANES
        }
        while i < len {
            let b = unsafe { *text.get_unchecked(i) };
            if !b.is_ascii_digit() {
                break;
            }
            i += 1
        }
        i
    }

    /// Skips ASCII identifier characters (a-z, A-Z, 0-9, _).
    #[inline(always)]
    pub fn skip_ascii_ident_continue(text: &[u8]) -> usize {
        let mut i = 0;
        let len = text.len();
        const LANES: usize = 32;
        while i + LANES <= len {
            let chunk = Simd::<u8, LANES>::from_slice(unsafe { text.get_unchecked(i..i + LANES) });
            let low = chunk.simd_ge(Simd::splat(b'a')) & chunk.simd_le(Simd::splat(b'z'));
            let up = chunk.simd_ge(Simd::splat(b'A')) & chunk.simd_le(Simd::splat(b'Z'));
            let dig = chunk.simd_ge(Simd::splat(b'0')) & chunk.simd_le(Simd::splat(b'9'));
            let und = chunk.simd_eq(Simd::splat(b'_'));
            let mask = low | up | dig | und;
            if !mask.all() {
                let not_mask = !mask;
                if let Some(idx) = not_mask.first_set() {
                    return i + idx;
                }
            }
            i += LANES
        }
        while i < len {
            let b = unsafe { *text.get_unchecked(i) };
            if !b.is_ascii_alphanumeric() && b != b'_' {
                break;
            }
            i += 1
        }
        i
    }

    /// Skips ASCII hex digits (0-9, a-f, A-F).
    #[inline(always)]
    pub fn skip_ascii_hexdigits(text: &[u8]) -> usize {
        let mut i = 0;
        let len = text.len();
        const LANES: usize = 32;
        while i + LANES <= len {
            let chunk = Simd::<u8, LANES>::from_slice(unsafe { text.get_unchecked(i..i + LANES) });
            let dig = chunk.simd_ge(Simd::splat(b'0')) & chunk.simd_le(Simd::splat(b'9'));
            let low = chunk.simd_ge(Simd::splat(b'a')) & chunk.simd_le(Simd::splat(b'f'));
            let up = chunk.simd_ge(Simd::splat(b'A')) & chunk.simd_le(Simd::splat(b'F'));
            let mask = dig | low | up;
            if !mask.all() {
                let not_mask = !mask;
                if let Some(idx) = not_mask.first_set() {
                    return i + idx;
                }
            }
            i += LANES
        }
        while i < len {
            let b = unsafe { *text.get_unchecked(i) };
            if !b.is_ascii_hexdigit() {
                break;
            }
            i += 1
        }
        i
    }

    /// Finds the first occurrence of any of the 4 bytes.
    #[inline(always)]
    pub fn find_first_of_4(text: &[u8], a: u8, b: u8, c: u8, d: u8) -> Option<usize> {
        let mut i = 0;
        let len = text.len();
        const LANES: usize = 32;

        while i + LANES <= len {
            let chunk = Simd::<u8, LANES>::from_slice(unsafe { text.get_unchecked(i..i + LANES) });
            let ma = chunk.simd_eq(Simd::splat(a));
            let mb = chunk.simd_eq(Simd::splat(b));
            let mc = chunk.simd_eq(Simd::splat(c));
            let md = chunk.simd_eq(Simd::splat(d));
            let mask = ma | mb | mc | md;
            if mask.any() {
                if let Some(idx) = mask.first_set() {
                    return Some(i + idx);
                }
            }
            i += LANES
        }

        while i < len {
            let byte = unsafe { *text.get_unchecked(i) };
            if byte == a || byte == b || byte == c || byte == d {
                return Some(i);
            }
            i += 1
        }
        None
    }

    /// Skips until the specified byte is found.
    #[inline(always)]
    pub fn skip_until(text: &[u8], target: u8) -> usize {
        let mut i = 0;
        let len = text.len();
        const LANES: usize = 32;

        while i + LANES <= len {
            let chunk = Simd::<u8, LANES>::from_slice(unsafe { text.get_unchecked(i..i + LANES) });
            let mask = chunk.simd_eq(Simd::splat(target));
            if mask.any() {
                if let Some(idx) = mask.first_set() {
                    return i + idx;
                }
            }
            i += LANES
        }

        while i < len {
            if unsafe { *text.get_unchecked(i) } == target {
                break;
            }
            i += 1
        }
        i
    }
}
