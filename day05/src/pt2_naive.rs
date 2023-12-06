use day05::*;
use rayon::prelude::*;

fn main() -> anyhow::Result<()> {
    let s = stdin_read()?;
    let (seeds, maps) = parse(&s);

    let seed_ranges = seeds
        .iter()
        .tuples()
        .map(|(s, l)| (*s..(*s + *l)))
        .collect_vec();

    let min = seed_ranges
        .par_iter()
        .map(|seed_range| {
            seed_range
                .clone()
                .into_par_iter()
                .map(|seed| maps.iter().fold(seed, |acc, m| m.map(acc)))
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    println!("{:?}", min);

    Ok(())
}
