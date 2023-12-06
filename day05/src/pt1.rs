use day05::*;

fn main() -> anyhow::Result<()> {
    let s = stdin_read()?;
    let (seeds, maps) = parse(&s);

    let min = seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |acc, m| m.map(acc)))
        .min()
        .unwrap();

    println!("{:?}", min);

    Ok(())
}
