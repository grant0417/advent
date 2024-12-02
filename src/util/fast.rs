/// This assumes the string is a valid u64
#[inline]
pub fn fast_parse_u64(s: &str) -> u64 {
    let mut n = 0;
    for b in s.bytes() {
        n = n * 10 + (b - b'0') as u64;
    }
    n
}
