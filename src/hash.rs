#[inline(always)]
pub fn pseudohash(bytes: &[u8]) -> f64 {
    let mut num = 1.0_f64;
    let len = bytes.len();

    for i in 0..len {
        let byte = bytes[len - 1 - i];
        let i1 = (len - i) as f64;

        num = ((1.1239285023_f64 / num) * (byte as f64) * std::f64::consts::PI
            + std::f64::consts::PI * i1).fract();
    }

    num
}

use std::fmt::{self, Write};

struct StackBuf {
    buf: [u8; 15],
    len: usize,
}

impl StackBuf {
    fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.buf[..self.len]) }
    }

    fn new() -> Self {
        Self {
            buf: [0; 15],
            len: 0,
        }
    }
}

impl fmt::Write for StackBuf {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        /*if self.len + bytes.len() > self.buf.len() {
            return Err(fmt::Error);
        }*/

        self.buf[self.len..self.len + s.len()].copy_from_slice(s.as_bytes());
        self.len += s.len();
        Ok(())
    }
}

#[inline(always)]
pub fn next(previous: f64) -> f64 {
    let next = (2.134453429141 + previous * 1.72431234).fract();

    let mut buf = StackBuf::new();
    write!(buf, "{next:.13}").unwrap();

    unsafe { buf.as_str().parse::<f64>().unwrap_unchecked() }

    // format!("{next:.13}").parse::<f64>().unwrap()
}
