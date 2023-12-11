use std::collections::VecDeque;

use day10::*;

/// https://en.wikipedia.org/wiki/Point_in_polygon#Ray_casting_algorithm is clearly more
/// appropriate. Leaving for posterity :)

#[allow(dead_code)]
fn pretty(grid: &Vec<Vec<Pipe>>) {
    for line in grid {
        for p in line {
            print!("{}", p.repr)
        }
        println!();
    }
}

// Expand the grid 2x so that flood fill can fill between gaps
fn explode(orig: &Vec<Vec<Pipe>>) -> Vec<Vec<Pipe>> {
    let mut new = vec![vec![Pipe::default(); orig[0].len() * 2]; orig.len() * 2];
    for (y, line) in orig.iter().enumerate() {
        for (x, p) in line.iter().enumerate() {
            new[y * 2][x * 2] = p.clone();
            for c in p.conns.iter() {
                let other = (d(x * 2, c.0), d(y * 2, c.1));
                new[other.1][other.0].conns.push((-c.0, -c.1));
            }
        }
    }

    // Set some pretty-print data
    for y in 0..new.len() {
        for x in 0..new[0].len() {
            let conns = new[y][x].conns.clone();
            let conns = conns.iter().sorted().collect_vec();
            let c = match conns.as_slice() {
                [(0, -1), (0, 1)] => '|',
                [(-1, 0), (1, 0)] => '-',
                [(0, -1), (1, 0)] => 'L',
                [(-1, 0), (0, -1)] => 'J',
                [(-1, 0), (0, 1)] => '7',
                [(0, 1), (1, 0)] => 'F',
                [] => '.',
                _ => '?',
            };
            new[y][x].repr = c;
        }
    }

    new
}

// Flood-fill (BFS) until touching outside or run out of places to fill (which -> inside).
fn is_piece_inner(
    grid: &Vec<Vec<Pipe>>,
    p: &(usize, usize),
    cache: &mut HashMap<(usize, usize), bool>,
) -> bool {
    if grid[p.1][p.0].repr != '.' {
        return false;
    }
    if cache.contains_key(p) {
        return cache[p];
    }

    // let grid = explode(grid);
    let mut q = VecDeque::new();
    q.push_back(*p);
    let mut seen = HashSet::new();
    seen.insert(*p);
    while let Some(node) = q.pop_front() {
        // println!("{:?} {}", node, grid[node.1][node.0].repr);
        if node.0 == 0 || node.0 == grid[0].len() - 1 || node.1 == 0 || node.1 == grid.len() - 1 {
            for pos in seen.into_iter() {
                cache.insert(pos, false);
            }
            return false;
        }
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let next = (d(node.0, dx), d(node.1, dy));
            if seen.contains(&next) {
                continue;
            }
            if grid[next.1][next.0].repr != '.' {
                continue;
            }

            seen.insert(next);
            q.push_back(next);
        }
    }
    for pos in seen.into_iter() {
        cache.insert(pos, true);
    }
    true
}

fn count_inner(grid: &Vec<Vec<Pipe>>, wall: &[(usize, usize)]) -> usize {
    let mut clean = vec![vec![Pipe::default(); grid[0].len()]; grid.len()];
    for (x, y) in wall.iter() {
        clean[*y][*x] = grid[*y][*x].clone();
    }
    let exploded = explode(&clean);

    let mut count = 0;
    let mut cache = HashMap::new();
    for y in 0..clean.len() {
        for x in 0..clean[0].len() {
            if is_piece_inner(&exploded, &(x * 2, y * 2), &mut cache) {
                count += 1;
            }
        }
    }
    count
}

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
            let count = count_inner(&input, &path);
            println!("{count}");
            break;
        }
    }

    Ok(())
}
