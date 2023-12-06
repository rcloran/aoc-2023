use day06::*;

fn main() -> anyhow::Result<()> {
    let input = extract_numbers::<_, usize>(stdin_lines()).collect_vec();

    let prod: usize = input[0]
        .iter()
        .zip(input[1].iter())
        .map(|(&time, &record)| (0..time).filter(|i| (time - i) * i > record).count())
        .product();

    println!("{:?}", prod);

    Ok(())
}
