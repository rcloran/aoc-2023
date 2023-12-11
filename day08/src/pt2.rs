use day08::*;

pub fn lcm2(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn main() -> anyhow::Result<()> {
    let s = stdin_read()?;
    let (_, (directions, nodes)) = parse(&s).n2a()?;

    let nodes: HashMap<&str, &Node> = HashMap::from_iter(nodes.iter().map(|n| (n.name, n)));

    let mut cur = nodes
        .values()
        .cloned()
        .filter(|n| n.name.ends_with('A'))
        .collect_vec();

    let mut history: Vec<HashMap<&str, usize>> = vec![HashMap::new(); cur.len()];
    let mut cycle_lengths: Vec<Option<usize>> = vec![None; cur.len()];

    'outer: for (x, dir) in directions.iter().cycle().enumerate() {
        let mut new = vec![];
        for (i, node) in cur.iter().enumerate() {
            let node = match dir {
                'L' => nodes[&node.left],
                'R' => nodes[&node.right],
                _ => unreachable!(),
            };
            new.push(node);

            if node.name.ends_with('Z') {
                if cycle_lengths[i].is_none() && history[i].contains_key(node.name) {
                    cycle_lengths[i] = Some(x - history[i][node.name]);
                    if cycle_lengths.iter().all(|&l| l.is_some()) {
                        break 'outer;
                    }
                }
                history[i].insert(node.name, x);
            }
        }
        cur = new;
    }

    let mut lcm = directions.len();
    for c in cycle_lengths {
        if let Some(c) = c {
            lcm = lcm2(lcm, c);
        }
    }
    println!("{}", lcm);
    Ok(())
}
