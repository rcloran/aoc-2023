use day02::*;

fn main() -> anyhow::Result<()> {
    let s = stdin_read()?;
    let (_, input) = parse(&s).n2a()?;

    let bag = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut id_sum = 0;
    'game: for game in input {
        for round in game.rounds {
            for cubes in round {
                if cubes.0 > *bag.get(cubes.1).unwrap_or(&0) {
                    continue 'game;
                }
            }
        }
        id_sum += game.id;
    }
    println!("{id_sum}");
    Ok(())
}
