use day07::*;

fn main() -> anyhow::Result<()> {
    let s = stdin_read()?;
    let (_, mut input) = parse(&s).n2a()?;

    for h in &mut input {
        h.set_jokered();
    }

    input.sort();

    let sum: usize = input
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) * h.bid as usize)
        .sum();
    println!("{sum}");

    Ok(())
}
