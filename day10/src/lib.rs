pub use util::prelude::*;

#[derive(Debug, Clone)]
pub struct Pipe {
    pub conns: Vec<(isize, isize)>,
    pub repr: char,
}

impl Default for Pipe {
    fn default() -> Self {
        Pipe {conns: vec![], repr: '.'}
    }
}

pub fn parse(input: &str) -> IResult<&str, Vec<Vec<Pipe>>> {
    all_consuming(terminated(separated_list1(newline, line), multispace0))(input)
}

pub fn line(input: &str) -> IResult<&str, Vec<Pipe>> {
    many1(pipe)(input)
}

pub fn pipe(input: &str) -> IResult<&str, Pipe> {
    let (input, c) = one_of("|-LJ7F.S")(input)?;

    let p = match c {
        '|' => Pipe {
            conns: vec![(0, -1), (0, 1)],
            repr: '|',
        },
        '-' => Pipe {
            conns: vec![(-1, 0), (1, 0)],
            repr: '-',
        },

        'L' => Pipe {
            conns: vec![(0, -1), (1, 0)],
            repr: 'L',
        },
        'J' => Pipe {
            conns: vec![(0, -1), (-1, 0)],
            repr: 'J',
        },

        '7' => Pipe {
            conns: vec![(0, 1), (-1, 0)],
            repr: '7',
        },
        'F' => Pipe {
            conns: vec![(0, 1), (1, 0)],
            repr: 'F',
        },

        '.' => Pipe { conns: vec![], repr: '.' },
        'S' => Pipe {
            conns: vec![(0, -1), (0, 1), (1, 0), (-1, 0)],
            repr: '*',
        },

        _ => unreachable!(),
    };

    Ok((input, p))
}

pub fn d(u: usize, i: isize) -> usize {
    (u as isize + i) as usize
}

fn step(
    grid: &[Vec<Pipe>],
    from: &(usize, usize),
    dir: &(isize, isize),
) -> Option<((usize, usize), (isize, isize))> {
    // println!("from={:?}, dir={:?}", from, dir);
    let next = (d(from.0, dir.0), d(from.1, dir.1));
    let mut from_idx: Option<usize> = None;
    for (i, c) in grid[next.1][next.0].conns.iter().enumerate() {
        if (d(next.0, c.0), d(next.1, c.1)) == *from {
            from_idx = Some(i);
            break;
        }
    }

    if let Some(from_idx) = from_idx {
        // This will be problematic if called going back in to start. Main loop should avoid that.
        if let Some(next_d) = grid[next.1][next.0]
            .conns
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != from_idx)
            .map(|(_, c)| c)
            .next()
        {
            // println!(" > {:?}, {:?}", next, next_d);
            Some((next, *next_d))
        } else {
            panic!("unexpected state")
        }
    } else {
        None
    }
}

pub fn follow_loop(
    grid: &[Vec<Pipe>],
    start: &(usize, usize),
    dir: &(isize, isize),
) -> Option<Vec<(usize, usize)>> {
    let mut cur = *start;
    let mut cur_dir = *dir;
    let mut r = vec![cur];
    step(grid, &cur, &cur_dir);
    while let Some(next) = step(grid, &cur, &cur_dir) {
        (cur, cur_dir) = next;
        if cur == *start {
            return Some(r)
        }
        r.push(cur);
    }
    None
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        assert_eq!(true, true);
        Ok(())
    }
}
