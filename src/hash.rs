use std::f64::consts::PI;
const C: f64 = 1.1239285023;

#[inline(always)]
pub fn pseudohash<const N: usize>(bytes: [u8; N]) -> f64 {
    let mut num = 1.0_f64;

    for i in 0..N {
        let byte = bytes[N - 1 - i];
        let i1 = (N - i) as f64;

        num = ((C / num) * (byte as f64) * PI + PI * i1).fract();
    }

    num
}

#[inline(always)]
pub fn pseudohash_erratic<const N: usize>(seed: [u8; N]) -> f64 {
    const PREFIX_LENGTH: usize = 7;
    const PREFIX: [u8; PREFIX_LENGTH] = *b"erratic";

    let mut num = 1.0_f64;

    for j in (0..N).rev() {
        let byte = seed[j];
        let i1 = (PREFIX_LENGTH + 1 + j) as f64;
        num = ((C / num) * (byte as f64) * PI + PI * i1).fract();
    }

    for k in (0..PREFIX_LENGTH).rev() {
        let byte = PREFIX[k];
        let i1 = (1 + k) as f64;
        num = ((C / num) * (byte as f64) * PI + PI * i1).fract();
    }

    num
}

use std::fmt::{self, Write};

struct StackBuf {
    buf: [u8; 15],
    len: usize,
}

impl StackBuf {
    #[inline(always)]
    fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.buf[..self.len]) }
    }

    #[inline(always)]
    fn new() -> Self {
        Self {
            buf: [0; 15],
            len: 0,
        }
    }
}
impl fmt::Write for StackBuf {
    #[inline(always)]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.buf[self.len..self.len + s.len()].copy_from_slice(s.as_bytes());
        self.len += s.len();
        Ok(())
    }
}

#[inline(always)]
pub fn next(previous: f64) -> f64 {
    let next = (2.134453429141 + previous * 1.72431234).fract();

    let mut buf = StackBuf::new();
    let _ = write!(buf, "{next:.13}");

    unsafe { lexical::parse(buf.as_str()).unwrap_unchecked() }
}
