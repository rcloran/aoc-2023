use day08::*;

fn main() -> anyhow::Result<()> {
    let s = stdin_read()?;
    let (_, (directions, nodes)) = parse(&s).n2a()?;

    let nodes: HashMap<&str, &Node> = HashMap::from_iter(nodes.iter().map(|n| (n.name, n)));

    let mut node = nodes["AAA"];
    let mut steps = 0;
    for dir in directions.iter().cycle() {
        steps += 1;
        node = match dir {
            'L' => nodes[&node.left],
            'R' => nodes[&node.right],
            _ => unreachable!(),
        };
        if node.name == "ZZZ" {
            break;
        }
    }

    println!("{}", steps);
    Ok(())
}
