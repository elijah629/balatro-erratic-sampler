pub const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ123456789"; // Alphanumeric minus 0 
const BASE: usize = CHARSET.len();

/// LENGTH MUST BE LESS THAN OR EQUAL TO 8
#[inline(always)]
pub const fn nth_combination<const LENGTH: usize>(mut n: usize) -> [u8; LENGTH] {
    let mut v = [CHARSET[0]; LENGTH];

    if LENGTH >= 1 { v[LENGTH-1] = CHARSET[n % BASE]; n /= BASE; }
    if LENGTH >= 2 { v[LENGTH-2] = CHARSET[n % BASE]; n /= BASE; }
    if LENGTH >= 3 { v[LENGTH-3] = CHARSET[n % BASE]; n /= BASE; }
    if LENGTH >= 4 { v[LENGTH-4] = CHARSET[n % BASE]; n /= BASE; }
    if LENGTH >= 5 { v[LENGTH-5] = CHARSET[n % BASE]; n /= BASE; }
    if LENGTH >= 6 { v[LENGTH-6] = CHARSET[n % BASE]; n /= BASE; }
    if LENGTH >= 7 { v[LENGTH-7] = CHARSET[n % BASE]; n /= BASE; }
    if LENGTH >= 8 { v[LENGTH-8] = CHARSET[n % BASE]; }

   v 
}
