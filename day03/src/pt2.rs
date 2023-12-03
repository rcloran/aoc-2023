use day03::*;

fn pn_idx(
    rows: usize,
    cols: usize,
    pns: &[(Span, usize)],
) -> Vec<Vec<Option<&(Span, usize)>>> {
    let mut pn_idx = vec![vec![None; cols]; rows];
    for pn in pns {
        let r = &mut pn_idx[pn.0.0];
        for j in pn.0.1..((pn.0.1 + pn.0.2).min(r.len())) {
            r[j] = Some(pn);
        }
    }
    pn_idx
}

fn gears(grid: &[Vec<u8>], pns: &[(Span, usize)]) -> Vec<(usize, usize)> {
    let mut r = vec![];
    let pn_idx = pn_idx(grid.len(), grid[0].len(), pns);

    for (i, l) in grid.iter().enumerate() {
        for (j, _) in l.iter().enumerate().filter(|(_, c)| **c == b'*') {
            let adjacent_pns = surrounds(&pn_idx, (i, j, 1))
                .flatten()
                .unique()
                .collect_vec();

            if adjacent_pns.len() == 2 {
                r.push((adjacent_pns[0].1, adjacent_pns[1].1));
            }
        }
    }

    r
}

fn main() -> anyhow::Result<()> {
    let grid = stdin_lines_u8().collect_vec();

    let part_numbers = find_part_numbers(&grid);
    println!(
        "{}",
        gears(&grid, &part_numbers)
            .into_iter()
            .map(|(a, b)| a * b)
            .sum::<usize>()
    );

    Ok(())
}
