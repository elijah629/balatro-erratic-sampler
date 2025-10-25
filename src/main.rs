use crate::{
    card::{Value, CARDS}, hash::{next, pseudohash}, rand::random_from_seed, seed::{nth_combination, CHARSET}
};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

mod card;
mod hash;
mod rand;
mod seed;

#[inline(always)]
fn is_valid(seed: &[u8]) -> bool {
    let hashed_seed_div_2 = pseudohash(seed) / 2.;

    let buf_end = 7 + seed.len();

    let mut buffer = [b'e', b'r', b'r', b'a', b't', b'i', b'c', 0, 0, 0, 0, 0, 0, 0, 0];
    buffer[7..buf_end].copy_from_slice(seed);

    let mut seed = next(pseudohash(&buffer[..buf_end]));

    for _ in 0..52 {
        let idx = (random_from_seed(seed / 2. + hashed_seed_div_2) * 52.) as usize;
        let card = &CARDS[idx];

        if card.0 >= Value::Ten  {
            return false;
        }

        seed = next(seed);
    }

    true
}

const RESUME_LENGTH: usize = 0;
const RESUME_INDEX: u64 = 0;

fn main() {
    for len in RESUME_LENGTH..=8 {
        let start = if len == RESUME_LENGTH { RESUME_INDEX } else { 0 };

        let total = CHARSET.len().pow(len as u32);
        let bar = ProgressBar::new(total as u64)
            .with_message(format!("Searching seeds of length {len}"))
            .with_style(
                ProgressStyle::with_template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) | ETA: {eta_precise} | {per_sec}")
                    .unwrap()
            ).with_position(start);

        (start as usize..total).into_par_iter().progress_with(bar).for_each(|n| {
            let s = &nth_combination(len, n)[..len];

            if is_valid(s) {
                let seed = unsafe { std::str::from_utf8_unchecked(s) };
                println!("FOUND seed: {seed}");

                std::process::exit(0);
            }
        });
    }

    println!("Search finished.");
}
