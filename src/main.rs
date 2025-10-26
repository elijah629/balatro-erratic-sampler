use crate::{
    card::CARDS, hash::{next, pseudohash, pseudohash_erratic, pseudohash_erratic_will_be_nan}, rand::random_from_seed, seed::{nth_combination, CHARSET}
};
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::time::Duration;

mod card;
mod hash;
mod rand;
mod seed;

/*const SEEDS: &[[u8; 8]] = &[
    *b"N9ER1XLW",
*b"SNML7XC8",
*b"PXJU9WJZ",
*b"LN27F5C3",
*b"6WG44S7Z",
*b"DHHHTLKL",
*b"LOSCHQBJ",
*b"SSU4L34S",
*b"SS44ACWE",
*b"SS7J9WXF",
*b"HOWRQJVZ",
*b"627SP6LE",
*b"J8CXY446",
*b"81N7QMOM",
*b"XEQH7CP9",
*b"3WBV4FSC",
*b"S4DM5V37",
*b"S4TMWWAE",
*b"YLFMCVYD",
*b"2W83JVKX",
*b"S641EUJT",
*b"S793AULP",
*b"QIB7UA76",
*b"36756M6I",
*b"KMGQ8TDZ",
*b"YT849CDU",
*b"9F47Q4YR",
*b"ZZFHQFDG",
*b"7LB2WVPK",
*b"IBVQGAOP",
*b"O6BRINCY",
*b"RDNCZ58P",
*b"O7CMD6K7",
*b"3B4W1TR7",
*b"MJA8I1NW",
*b"9PCOBR2D",
*b"TO7WNQA5",
*b"PDHOE636",
*b"RJ8NRSIS",
];*/

fn main() {
    // Run all lengths with each length's codegen optimized
   // test_length::<0>();
    //test_length::<1>();
    //test_length::<2>();
    //test_length::<3>();
    //test_length::<4>();
    //test_length::<5>();
    //test_length::<6>();
    //test_length::<7>();
    test_length::<8>();

    /*for &seed in SEEDS {
        let a = will_be_nan(seed);
        let b = pseudohash_erratic_will_be_nan(seed);

        assert_eq!(a, b);
   //     println!("{}: {}", unsafe { str::from_utf8_unchecked(seed) }, categorize_seed::<8>(*seed));
    }*/

    println!("Search finished.");
}

/*fn categorize_seed<const N: usize>(seed: [u8; N]) -> &'static str {
    let half_hashed = pseudohash(seed) * 0.5;
    let mut seed = next(pseudohash_erratic(seed)); // NaN if hash_erratic
    // is NaN


    for _ in 0..52 {
        let idx = (random_from_seed((0.5f64).mul_add(seed, half_hashed)) * 52.) as usize;

        if idx != CARDS_LAST_INDEX {
            let parity = (CARDS[idx].0 as u8) & 1;

            return if parity == 0 {
                "Even"
            } else {
                "Odd"
            }
        }

        seed = next(seed);
    }

    "NaN"
}*/

#[inline(always)]
fn is_valid<const N: usize>(seed: [u8; N]) -> bool {
    pseudohash_erratic_will_be_nan(seed)
    /*let half_hashed = pseudohash(seed) * 0.5;
    let mut seed = next(pseudohash_erratic(seed));

    let idx = (random_from_seed((0.5f64).mul_add(seed, half_hashed)) * 52.) as usize;
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

    // Fast fail. Check if deck is all 10 of spades
    /*for _ in 0..52 {
        let idx = (random_from_seed((0.5f64).mul_add(seed, half_hashed)) * 52.) as usize;

        if idx != CARDS_LAST_INDEX {
            return false;
        }

        seed = next(seed);
    }*/

   // true
}

#[inline(always)]
fn test_length<const LENGTH: usize>() {
    let total =  CHARSET.len().pow(LENGTH as u32);

    let bar = ProgressBar::new(total as u64)
            .with_message(format!("Searching seeds of length {LENGTH}"))
            .with_style(
                ProgressStyle::with_template("{msg} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) | ETA: {eta_precise} | {per_sec}")
                    .unwrap()
            );

    bar.enable_steady_tick(Duration::from_millis(100));

    (0..total).into_par_iter().progress_with(bar).for_each(|n| {
        let s = nth_combination::<{ LENGTH }>(n);

        if is_valid(s) {
            let seed = unsafe { std::str::from_utf8_unchecked(&s) };
            println!("FOUND seed: {seed} with combination {n}");

            // keep going through all 2 trillion something seeds
            // std::process::exit(0);
        }
    });
}
