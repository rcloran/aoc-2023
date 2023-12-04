use day04::*;

fn main() -> anyhow::Result<()> {
    let s = stdin_read()?;
    let (_, cards) = parse(&s).n2a()?;

    let mut total = 0;
    for card in cards {
        let matches = card.matches();
        total += match matches {
            0 => 0,
            _ => 2usize.pow(matches as u32 - 1),
        };
    }

    println!("{total}");
    Ok(())
}
