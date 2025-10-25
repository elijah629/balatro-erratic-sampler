#[inline(always)]
pub fn random_from_seed(mut d: f64) -> f64 {
    const K: [u32; 4] = [63, 58, 55, 47];
    const Q: [u32; 4] = [31, 19, 24, 21];
    const S: [u32; 4] = [18, 28, 7, 8];
    const TOP_MASKS: [u64; 4] = [
        (!0u64) << (64 - K[0]),
        (!0u64) << (64 - K[1]),
        (!0u64) << (64 - K[2]),
        (!0u64) << (64 - K[3]),
    ];

    #[inline(always)]
    fn transform(z: u64, k: u32, q: u32, s: u32, top_mask: u64) -> u64 {
        (((z << q) ^ z) >> (k - s)) ^ ((z & top_mask) << s)
    }

    let mut u = [0u64; 4];
    let mut r: u32 = 0x1109_0601;

    for item in &mut u {
        let shift = r & 0xFF;
        r >>= 8;
        d = d * std::f64::consts::PI + std::f64::consts::E;
        let mut bits = d.to_bits();
        bits = bits.max(1u64 << shift);
        *item = bits;
    }

    for _ in 0..10 {
        for i in 0..4 {
            u[i] = transform(u[i], K[i], Q[i], S[i], TOP_MASKS[i]);
        }
    }

    let mut r = 0u64;
    for i in 0..4 {
        u[i] = transform(u[i], K[i], Q[i], S[i], TOP_MASKS[i]);
        r ^= u[i];
    }

    let r = (r & 0x000F_FFFF_FFFF_FFFFu64) | 0x3FF0_0000_0000_0000u64;
    f64::from_bits(r) - 1.0
}
