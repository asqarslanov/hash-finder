use std::fmt::Write as _;
use std::num::Wrapping;

/// Hashes given bytes with the SHA-256 algorithm (inspired by
/// [this Stack Overflow answer](https://stackoverflow.com/a/78143703/16464166)).
///
/// Note: a typical SHA-256 code would have the datatype `[u8; 32]`.
/// However, this function returns 64 elements.
/// You may think of the return type as `[u4; 64]`.
///
/// The elements are transformed like this:
/// - `[a, b, c]` (typical)
/// - `[a / 16, a % 16, b / 16, b % 16, c / 16, c % 16]` (this function).
///
/// The reason for this is to make each element correspond
/// to a character in the string representation of the hash.
#[allow(clippy::missing_panics_doc)]
pub fn digest(bytes: &[u8]) -> [u8; 64] {
    #[allow(clippy::unreadable_literal)]
    let mut current_hash = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    let mut offset = 0;
    while bytes.len() - offset >= 64 {
        compress_round(&mut current_hash, &bytes[offset..offset + 64]);
        offset += 64;
    }

    let offset_bytes = &bytes[offset..];

    let mut buffer = [0; 64];
    buffer[..offset_bytes.len()].copy_from_slice(offset_bytes);
    buffer[bytes.len()] = 0x80;
    if bytes.len() < 56 {
        buffer[bytes.len() + 1..56].fill(0);
    } else {
        buffer[bytes.len() + 1..].fill(0);
        compress_round(&mut current_hash, &buffer);
        buffer[..56].fill(0);
    }

    let total_size = offset_bytes.len() as u64;
    buffer[56..].copy_from_slice(&(total_size << 3).to_be_bytes());
    compress_round(&mut current_hash, &buffer);

    <[u8; 64]>::try_from(
        current_hash
            .map(|it| it.to_be_bytes().map(|it| [it / 16, it % 16]))
            .as_flattened()
            .as_flattened(),
    )
    .expect(
        "`[u32; 8]` should be convertable to `[u8; 32] by itself, \
         but we correspond each element to a pair (`it / 16`, `it % 16`)`, \
         so this should result in 64 elements",
    )
}

/// Converts a hash calculated with the [`digest`] function to its string
/// representation (lowercase).
pub fn format(hash: &[u8; 64]) -> String {
    hash.chunks(2)
        .map(|chunk| 16 * chunk[0] + chunk[1])
        .fold(String::new(), |mut output, byte| {
            write!(output, "{byte:02x}").expect("writing to a string shouldn't fail");
            output
        })
}

fn compress_round(input: &mut [u32; 8], block: &[u8]) {
    #[allow(clippy::unreadable_literal)]
    let round_constants = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ]
    .map(Wrapping);

    let block = <[_; 16]>::try_from(
        block[..64]
            .chunks(4)
            .map(|c| {
                u32::from_be_bytes(
                    <[u8; 4]>::try_from(c)
                        .expect("chunks of size 4 should be convertable to arrays of size 4"),
                )
            })
            .collect::<Vec<_>>(),
    )
    .expect("chunks of 4 from 64 elements should result in 16 elements");

    let mut buffer = input.map(Wrapping);

    for (constant, expanded) in round_constants.into_iter().zip(expander(&block)) {
        let s_0 = Wrapping(sig_zero(buffer[0].0));
        let s_1 = Wrapping(sig_one(buffer[4].0));

        let c = Wrapping(choose(buffer[4].0, buffer[5].0, buffer[6].0));
        let m = Wrapping(majority(buffer[0].0, buffer[1].0, buffer[2].0));

        let t_1 = buffer[7] + s_1 + c + constant + expanded;
        let t_2 = s_0 + m;

        buffer.rotate_right(1);
        buffer[0] = t_1 + t_2;
        buffer[4] += t_1;
    }

    *input = [
        (Wrapping(input[0]) + buffer[0]).0,
        (Wrapping(input[1]) + buffer[1]).0,
        (Wrapping(input[2]) + buffer[2]).0,
        (Wrapping(input[3]) + buffer[3]).0,
        (Wrapping(input[4]) + buffer[4]).0,
        (Wrapping(input[5]) + buffer[5]).0,
        (Wrapping(input[6]) + buffer[6]).0,
        (Wrapping(input[7]) + buffer[7]).0,
    ];
}

fn expander(bits: &[u32; 16]) -> [Wrapping<u32>; 64] {
    let mut result = [0; 64];

    result[..16].copy_from_slice(bits);

    for i in 16..64 {
        result[i] = result[i - 16]
            .wrapping_add(sigma_zero(result[i - 15]))
            .wrapping_add(result[i - 7])
            .wrapping_add(sigma_one(result[i - 2]));
    }

    result.map(Wrapping)
}

const fn sigma_zero(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

const fn sigma_one(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

const fn choose(e: u32, f: u32, g: u32) -> u32 {
    (e & f) ^ (!e & g)
}

const fn majority(a: u32, b: u32, c: u32) -> u32 {
    (a & b) ^ (a & c) ^ (b & c)
}

const fn sig_one(e: u32) -> u32 {
    e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25)
}

const fn sig_zero(a: u32) -> u32 {
    a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_strings() -> impl IntoIterator<Item = &'static [u8]> {
        [
            "0",
            "-1",
            "abc",
            "Hello, world!",
            "42",
            "blazingly fast",
            "инглиш не понимаем",
            "Ё!№jkafd#$",
            "☺️🙂😊😀😁",
        ]
        .map(str::as_bytes)
    }

    #[test]
    fn hash_generation() {
        for it in test_strings() {
            assert_eq!(format(&digest(it)), sha256::digest(it));
        }
    }

    // To be fair, I should've also tested
    // the `digest` and `format` functions separately.
    // However, their outputs/inputs are not very human-readable,
    // which would result in fabricated tests generated by this code itself.
}
