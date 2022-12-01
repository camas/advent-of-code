pub fn md5_string(data: &str) -> String {
    let digest = md5(data.as_bytes());

    const CHARS: &[u8] = b"0123456789abcdef";
    let mut v = Vec::with_capacity(digest.len() * 2);
    for b in digest.iter() {
        v.push(CHARS[(b >> 4) as usize]);
        v.push(CHARS[(b & 0xf) as usize]);
    }

    unsafe { String::from_utf8_unchecked(v) }
}

/// Rust implementation of MD5
///
/// https://en.wikipedia.org/wiki/MD5#Algorithm
#[allow(clippy::many_single_char_names)]
pub fn md5(input: &[u8]) -> [u8; 16] {
    // Constants
    const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613,
        0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193,
        0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d,
        0x2441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
        0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122,
        0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa,
        0xd4ef3085, 0x4881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244,
        0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
        0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb,
        0xeb86d391,
    ];
    const S: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];

    // Pad data
    let mut data = Vec::from(input);
    data.push(0b1000_0000);
    let data_len = data.len() % 64;
    let add_count = if data_len > 56 {
        56 + (64 - data_len)
    } else {
        56 - data_len
    };
    data.extend(vec![0; add_count]);
    data.extend((input.len() as u64 * 8).to_le_bytes());
    debug_assert_eq!(data.len() % 64, 0);

    // Hash
    let mut a0: u32 = 0x67452301;
    let mut b0: u32 = 0xefcdab89;
    let mut c0: u32 = 0x98badcfe;
    let mut d0: u32 = 0x10325476;
    for chunk in data.chunks(64) {
        let m = chunk
            .chunks(4)
            .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .collect::<Vec<_>>();
        let mut a = a0;
        let mut b = b0;
        let mut c = c0;
        let mut d = d0;
        for i in 0..64 {
            let (f, g) = match i {
                0..=15 => {
                    let f = (b & c) | (!b & d);
                    let g = i;
                    (f, g)
                }
                16..=31 => {
                    let f = (d & b) | (!d & c);
                    let g = ((5 * i) + 1) % 16;
                    (f, g)
                }
                32..=47 => {
                    let f = b ^ c ^ d;
                    let g = ((3 * i) + 5) % 16;
                    (f, g)
                }
                48..=63 => {
                    let f = c ^ (b | !d);
                    let g = (7 * i) % 16;
                    (f, g)
                }
                _ => unreachable!(),
            };
            let f = f.wrapping_add(a).wrapping_add(K[i]).wrapping_add(m[g]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(S[i]));
        }
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }

    let mut digest = [0_u8; 16];
    digest[..4].copy_from_slice(&a0.to_le_bytes());
    digest[4..8].copy_from_slice(&b0.to_le_bytes());
    digest[8..12].copy_from_slice(&c0.to_le_bytes());
    digest[12..].copy_from_slice(&d0.to_le_bytes());
    digest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5_known() {
        assert_eq!(
            md5_string("The quick brown fox jumps over the lazy dog"),
            "9e107d9d372bb6826bd81d3542a419d6"
        );
        assert_eq!(
            md5_string("The quick brown fox jumps over the lazy dog."),
            "e4d909c290d0fb1ca068ffaddf22cbd0"
        );
        assert_eq!(md5_string(""), "d41d8cd98f00b204e9800998ecf8427e");
    }
}
