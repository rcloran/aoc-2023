use day10::*;


fn main() -> anyhow::Result<()> {
    let s = stdin_read()?;
    let (_, input) = parse(&s).n2a()?;

    // Parsing maps start to connecting in all directions
    let start = input
        .iter()
        .map(|l| l.iter().position(|p| p.conns.len() == 4))
        .enumerate()
        .flat_map(|(y, x)| x.map(|x| (x, y)))
        .collect_vec();
    let start = start[0];

    let start_node = &input[start.1][start.0];

    for dir in start_node.conns.iter() {
        if let Some(path) = follow_loop(&input, &start, dir) {
            println!("{}", path.len() / 2);
            break
        }
        
    }

    Ok(())
}
