use day04::*;

fn main() -> anyhow::Result<()> {
    let s = stdin_read()?;
    let (_, input) = parse(&s).n2a()?;

    let mut copies = vec![1; input.len()];
    for (i, card) in input.into_iter().enumerate() {
        for j in (i+1)..=(i + card.matches()) {
            copies[j] += copies[i];
        }
        
    };

    println!("{}", copies.into_iter().sum::<usize>());
    Ok(())
}
