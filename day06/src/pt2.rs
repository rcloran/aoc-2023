use day06::*;

fn main() -> anyhow::Result<()> {
    let input = extract_numbers::<_, String>(stdin_lines()).collect_vec();

    let time: usize = input[0].iter().join("").parse()?;
    let record: usize = input[1].iter().join("").parse()?;

    println!("{}", (0..time).filter(|i| (time - i) * i > record).count());

    Ok(())
}
