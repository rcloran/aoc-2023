use day09::*;

fn main() -> anyhow::Result<()> {
    let input = extract_numbers::<_, isize>(stdin_lines());

    let sum: isize = input
        .into_iter()
        .map(|l| succ_diff(&l, &|l, below| l.last().unwrap() + below))
        .sum();
    println!("{sum}");

    Ok(())
}
