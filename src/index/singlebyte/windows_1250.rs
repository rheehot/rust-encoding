// AUTOGENERATED FROM index-windows-1250.txt, ORIGINAL COMMENT FOLLOWS:
//
// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/
//
// For details on index index-windows-1250.txt see the Encoding Standard
// https://encoding.spec.whatwg.org/
//
// Identifier: 0669455a7a1c70ba6003ea737991e8ee9adc455125c13cfe6705a361358de5fa
// Date: 2014-12-19

#[allow(dead_code)] const X: u16 = 0xffff;

const FORWARD_TABLE: &'static [u16] = &[
    8364, 129, 8218, 131, 8222, 8230, 8224, 8225, 136, 8240, 352, 8249, 346,
    356, 381, 377, 144, 8216, 8217, 8220, 8221, 8226, 8211, 8212, 152, 8482,
    353, 8250, 347, 357, 382, 378, 160, 711, 728, 321, 164, 260, 166, 167, 168,
    169, 350, 171, 172, 173, 174, 379, 176, 177, 731, 322, 180, 181, 182, 183,
    184, 261, 351, 187, 317, 733, 318, 380, 340, 193, 194, 258, 196, 313, 262,
    199, 268, 201, 280, 203, 282, 205, 206, 270, 272, 323, 327, 211, 212, 336,
    214, 215, 344, 366, 218, 368, 220, 221, 354, 223, 341, 225, 226, 259, 228,
    314, 263, 231, 269, 233, 281, 235, 283, 237, 238, 271, 273, 324, 328, 243,
    244, 337, 246, 247, 345, 367, 250, 369, 252, 253, 355, 729,
]; // 128 entries

/// Returns the index code point for pointer `code` in this index.
#[inline]
pub fn forward(code: u8) -> u16 {
    FORWARD_TABLE[(code - 0x80) as usize]
}

#[cfg(not(feature = "no-optimized-legacy-encoding"))]
const BACKWARD_TABLE_LOWER: &'static [u8] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 193, 194, 0, 196, 0, 0, 199, 0,
    201, 0, 203, 0, 205, 206, 0, 0, 0, 0, 211, 212, 0, 214, 215, 0, 0, 218, 0,
    220, 221, 0, 223, 0, 225, 226, 0, 228, 0, 0, 231, 0, 233, 0, 235, 0, 237,
    238, 0, 0, 0, 0, 243, 244, 0, 246, 247, 0, 0, 250, 0, 252, 253, 0, 0, 129,
    0, 131, 0, 0, 0, 0, 136, 0, 0, 0, 0, 0, 0, 0, 144, 0, 0, 0, 0, 0, 0, 0,
    152, 0, 0, 0, 0, 0, 0, 0, 160, 0, 0, 0, 164, 0, 166, 167, 168, 169, 0, 171,
    172, 173, 174, 0, 176, 177, 0, 0, 180, 181, 182, 183, 184, 0, 0, 187, 0, 0,
    0, 0, 0, 0, 0, 161, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 162,
    255, 0, 178, 0, 189, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 153, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 150, 151, 0, 0, 0,
    145, 146, 130, 0, 147, 148, 132, 0, 134, 135, 149, 0, 0, 0, 133, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 137, 0, 0, 0, 0, 0, 0, 0, 0, 139, 155, 0, 0, 0, 0, 0,
    195, 227, 165, 185, 198, 230, 0, 0, 0, 0, 200, 232, 207, 239, 208, 240, 0,
    0, 0, 0, 0, 0, 202, 234, 204, 236, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 197, 229, 0, 0, 188, 190,
    0, 163, 179, 209, 241, 0, 0, 210, 242, 0, 0, 0, 0, 0, 0, 0, 213, 245, 0, 0,
    192, 224, 0, 0, 216, 248, 140, 156, 0, 0, 170, 186, 138, 154, 222, 254,
    141, 157, 0, 0, 0, 0, 0, 0, 0, 0, 217, 249, 219, 251, 0, 0, 0, 0, 0, 0, 0,
    143, 159, 175, 191, 142, 158, 0,
]; // 495 entries

#[cfg(not(feature = "no-optimized-legacy-encoding"))]
const BACKWARD_TABLE_UPPER: &'static [u16] = &[
    0, 0, 126, 63, 368, 431, 0, 0, 0, 0, 0, 186, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 306, 0, 216, 0, 261,
]; // 133 entries

/// Returns the index pointer for code point `code` in this index.
#[inline]
#[cfg(not(feature = "no-optimized-legacy-encoding"))]
pub fn backward(code: u32) -> u8 {
    let offset = (code >> 6) as usize;
    let offset = if offset < 133 {BACKWARD_TABLE_UPPER[offset] as usize} else {0};
    BACKWARD_TABLE_LOWER[offset + ((code & 63) as usize)]
}

/// Returns the index pointer for code point `code` in this index.
#[cfg(feature = "no-optimized-legacy-encoding")]
pub fn backward(code: u32) -> u8 {
    if code > 8482 || ((0x10003u32 >> (code >> 9)) & 1) == 0 { return 0; }
    let code = code as u16;
    for i in 0..0x80 {
        if FORWARD_TABLE[i as usize] == code { return 0x80 + i; }
    }
    0
}

#[cfg(test)]
single_byte_tests!(
    mod = windows_1250
);
