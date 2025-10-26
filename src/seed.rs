pub const CHARSET: [u8; 35] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ123456789"; // Alphanumeric minus 0
const BASE: u128 = CHARSET.len() as u128;

const INV_BASE: u128 = (1 << 64) / BASE + 1;

#[inline(always)]
const fn divmod_base(n: usize) -> (usize, usize) {
    let q = (n as u128 * INV_BASE) >> 64;
    let r = (n as u128 - q * BASE) as usize;
    (q as usize, r)
}

/// Saftey
/// Length must be less than or equal to 8
#[inline(always)]
pub fn nth_combination<const LENGTH: usize>(mut n: usize) -> [u8; LENGTH] {
    let mut v = [CHARSET[0]; LENGTH];

    // SAFTEY: Compiler cannot infer that the divmod modulo will be less than BASE, so we
    // must get_unchecked to avoid unnessicary checks -- boosting speed
    unsafe {
        if LENGTH >= 1 {
            let (d0, m0) = divmod_base(n);
            v[LENGTH - 1] = *CHARSET.get_unchecked(m0);
            n = d0;
        }
        if LENGTH >= 2 {
            let (d1, m1) = divmod_base(n);
            v[LENGTH - 2] = *CHARSET.get_unchecked(m1);
            n = d1;
        }
        if LENGTH >= 3 {
            let (d2, m2) = divmod_base(n);
            v[LENGTH - 3] = *CHARSET.get_unchecked(m2);
            n = d2;
        }
        if LENGTH >= 4 {
            let (d3, m3) = divmod_base(n);
            v[LENGTH - 4] = *CHARSET.get_unchecked(m3);
            n = d3;
        }
        if LENGTH >= 5 {
            let (d4, m4) = divmod_base(n);
            v[LENGTH - 5] = *CHARSET.get_unchecked(m4);
            n = d4;
        }
        if LENGTH >= 6 {
            let (d5, m5) = divmod_base(n);
            v[LENGTH - 6] = *CHARSET.get_unchecked(m5);
            n = d5;
        }
        if LENGTH >= 7 {
            let (d6, m6) = divmod_base(n);
            v[LENGTH - 7] = *CHARSET.get_unchecked(m6);
            n = d6;
        }
        if LENGTH >= 8 {
            let (_, m7) = divmod_base(n);
            v[LENGTH - 8] = *CHARSET.get_unchecked(m7);
        }
    }

    v
}

