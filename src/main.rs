use std::time::Duration;
use crate::{
    card::{CARDS_LAST_INDEX}, hash::{next, pseudohash, pseudohash_erratic}, rand::random_from_seed, seed::{nth_combination, CHARSET}
};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

mod card;
mod hash;
mod rand;
mod seed;

fn main() {
    test_length::<0>();
    test_length::<1>();
    test_length::<2>();
    test_length::<3>();
    test_length::<4>(); 
    test_length::<5>();
    test_length::<6>();
    test_length::<7>();
    test_length::<8>();

    println!("Search finished.");
}

#[inline(always)]
fn is_valid<const N: usize>(seed: [u8; N]) -> bool {
    let half_hashed = pseudohash(seed) * 0.5;
    let mut seed = next(pseudohash_erratic(seed));

    /*let idx = (random_from_seed((0.5f64).mul_add(seed, half_hashed)) * 52.) as usize;
    let parity = CARDS[idx].0 as u8 & 1;

    seed = next(seed);

    for _ in 1..52 {
        let idx = (random_from_seed((0.5f64).mul_add(seed, half_hashed)) * 52.) as usize;
        let card = &CARDS[idx];

        if card.0 as u8 & 1 != parity {
            return false;
        }

        seed = next(seed);
    }*/

    for _ in 0..52 {
        let idx = (random_from_seed((0.5f64).mul_add(seed, half_hashed)) * 52.) as usize;

        if idx != CARDS_LAST_INDEX {
            return false;
        }

        seed = next(seed);
    }

    true
}

#[inline(always)]
fn test_length<const LENGTH: usize>() {
    let total = CHARSET.len().pow(LENGTH as u32);
    
    let bar = ProgressBar::new(total as u64)
            .with_message(format!("Searching seeds of length {LENGTH}"))
            .with_style(
                ProgressStyle::with_template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) | ETA: {eta_precise} | {per_sec}")
                    .unwrap()
            );

            bar.enable_steady_tick(Duration::from_millis(100));

          (0..total).into_par_iter().progress_with(bar).for_each(|n| {
            let s = nth_combination::<{LENGTH}>(n);

            if is_valid(s) {
                let seed = unsafe { std::str::from_utf8_unchecked(&s) };
                println!("FOUND seed: {seed} with combination {n}");

                // std::process::exit(0);
            }
        });


}
