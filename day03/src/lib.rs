pub use util::prelude::*;

pub type Span = (usize, usize, usize);

pub fn find_numbers(grid: &[Vec<u8>]) -> Vec<(Span, usize)> {
    fn n(s: &[u8]) -> usize {
        // We've already checked that each position is a digit, so it's always valid UTF-8, and
        // will always parse to a number.
        unsafe { std::str::from_utf8_unchecked(s).parse().unwrap() }
    }

    let re = regex::bytes::Regex::new("[0-9]+").unwrap();

    let mut r = vec![];
    for (i, l) in grid.iter().enumerate() {
        for m in re.find_iter(l) {
            r.push(((i, m.start(), m.len()), n(m.as_bytes())))
        }
    }
    r
}

/// Find the values surrounding a span of values -- the "border"
pub fn surrounds<T>(
    grid: &[Vec<T>],
    (row, start, len): (usize, usize, usize),
) -> impl Iterator<Item = T> + '_
where
    T: Copy,
{
    let r1 = row.saturating_sub(1);
    let r2 = (row + 1).min(grid.len() - 1);
    let c1 = start.saturating_sub(1);
    let c2 = (start + len).min(grid[0].len() - 1);

    (c1..=c2)
        .cartesian_product(r1..=r2)
        .map(|(c, r)| grid[r][c])
}

pub fn is_part_number(grid: &[Vec<u8>], idx: Span) -> bool {
    surrounds(grid, idx).any(|c| !b".0123456789".contains(&c))
}

pub fn find_part_numbers(grid: &[Vec<u8>]) -> Vec<(Span, usize)> {
    find_numbers(grid)
        .into_iter()
        .filter(|n| is_part_number(grid, n.0))
        .collect()
}
