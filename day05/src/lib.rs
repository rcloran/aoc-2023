use std::ops::Range;

use regex::Regex;
pub use util::prelude::*;

pub struct RangeMap {
    pub ranges: Vec<(Range<usize>, Range<usize>)>,
}

impl RangeMap {
    pub fn try_map(&self, input: usize) -> Option<usize> {
        for (k, v) in self.ranges.iter() {
            if k.contains(&input) {
                return Some((input - k.start) + v.start);
            }
        }
        None
    }

    pub fn map(&self, input: usize) -> usize {
        if let Some(v) = self.try_map(input) {
            v
        } else {
            input
        }
    }
}

pub fn parse(input: &str) -> (Vec<usize>, Vec<RangeMap>) {
    let mut parts = input.split("\n\n");
    let num = Regex::new("[0-9]+").unwrap();
    let seeds = num
        .find_iter(parts.next().unwrap())
        .map(|m| m.as_str().parse().unwrap())
        .collect_vec();

    let mut maps = vec![];
    for mappart in parts {
        let mut lines = mappart.lines();
        let _header = lines.next();
        let ranges = extract_numbers(lines).map(|l| {
            let out_range = l[0]..(l[0] + l[2]);
            let in_range = l[1]..(l[1] + l[2]);
            (in_range, out_range)
        }).collect_vec();
        maps.push(RangeMap { ranges });
    }

    (seeds, maps)
}
