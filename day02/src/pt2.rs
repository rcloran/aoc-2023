use day02::*;

fn main() -> anyhow::Result<()> {
    let s = stdin_read()?;
    let (_, input) = parse(&s).n2a()?;

    let mut power_sum = 0;
    for game in input {
        let mut max: HashMap<&str, usize> = HashMap::new();
        for round in game.rounds {
            for cubes in round {
                let e = max.entry(cubes.1).or_default();
                *e = cubes.0.max(*e);
            }
        }
        // `max.values().product()` works fine for my input, but if any are 0, it would be
        // incorrect.
        let power = max.get("red").unwrap_or(&0)
            * max.get("blue").unwrap_or(&0)
            * max.get("green").unwrap_or(&0);
        power_sum += power;
    }
    println!("{power_sum}");
    Ok(())
}
