pub const CHARSET: &[u8] = b"ABCDEFGHIJKLMNPQRSTUVWXYZ123456789"; // Alphanumeric minus 0 and O

pub fn nth_combination(len: usize, mut n: usize) -> [u8; 8] {
    let base = CHARSET.len();
    let mut buf = [CHARSET[0]; 8];

    for i in (0..len).rev() {
        buf[i] = CHARSET[n % base];
        n /= base;
    }

    buf
}
