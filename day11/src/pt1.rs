use day11::*;

fn main() -> anyhow::Result<()> {
    let s = stdin_read()?;
    let (_, input) = parse(&s).n2a()?;

    let weights = dark_energy(&input, 2);

    let r: usize = galaxies(&input)
        .combinations(2)
        .map(|ab| dist(&ab[0], &ab[1], &weights))
        .sum();
    println!("{:?}", r);

    Ok(())
}
