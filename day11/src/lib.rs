pub use util::prelude::*;

use nom::Parser;

pub fn galaxies(list: &[Vec<bool>]) -> impl Iterator<Item = (usize, usize)> + '_ {
    list.iter().enumerate().flat_map(|(y, line)| {
        line.iter()
            .enumerate()
            .filter_map(move |(x, g)| g.then_some((x, y)))
    })
}

pub fn dark_energy(input: &[Vec<bool>], expansion: usize) -> Vec<Vec<usize>> {
    let width = input[0].len();
    let mut new = vec![];
    for line in input.iter() {
        if line.iter().all(|v| !v) {
            new.push(vec![expansion; width]);
        } else {
            new.push(vec![1; width]);
        }
    }
    for i in 0..new[0].len() {
        if (0..input.len()).map(|j| input[j][i]).all(|v| !v) {
            for line in &mut new {
                line[i] = expansion;
            }
        }
    }
    new
}

pub fn dist(a: &(usize, usize), b: &(usize, usize), weights: &[Vec<usize>]) -> usize {
    let min_x = a.0.min(b.0);
    let max_x = a.0.max(b.0);
    let min_y = a.1.min(b.1);
    let max_y = a.1.max(b.1);

    (min_y..max_y).map(|y| weights[y][min_x]).sum::<usize>()
        + (min_x..max_x).map(|x| weights[max_y][x]).sum::<usize>()
}

pub fn parse(input: &str) -> IResult<&str, Vec<Vec<bool>>> {
    all_consuming(terminated(separated_list1(newline, line), multispace0))(input)
}

pub fn line(input: &str) -> IResult<&str, Vec<bool>> {
    many1(one_of(".#").map(|c| match c {
        '.' => false,
        '#' => true,
        _ => unreachable!(),
    }))(input)
}
