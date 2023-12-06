use std::ops::Range;

use day05::*;

fn maybe_split(seed: &Range<usize>, map_inp: &Range<usize>) -> Vec<Range<usize>> {
    if map_inp.contains(&seed.start) {
        if map_inp.contains(&seed.end) {
            vec![seed.clone()]
        } else {
            vec![seed.start..map_inp.end, map_inp.end..seed.end]
        }
    } else if map_inp.contains(&seed.end) {
        // Fully contained case is covered above, so we have an overlapping case
        vec![seed.start..map_inp.start, map_inp.start..seed.end]
    } else if seed.contains(&map_inp.start) {
        vec![
            seed.start..map_inp.start,
            map_inp.start..map_inp.end,
            map_inp.end..seed.end,
        ]
    } else {
        vec![seed.clone()]
    }
}

fn main() -> anyhow::Result<()> {
    let s = stdin_read()?;
    let (seeds, maps) = parse(&s);

    let mut vals = seeds
        .iter()
        .tuples()
        .map(|(seed_start, seed_len)| (*seed_start..(*seed_start + *seed_len)))
        .collect_vec();

    for map in maps {
        // First make sure no value ranges overlap with the start/end of input ranges of the map
        vals = map.ranges.iter().fold(vals, |vals, (inp, _)| {
            vals.iter().flat_map(|v| maybe_split(v, inp)).collect()
        });

        // Then do the remapping
        vals = vals
            .into_iter()
            .map(|val| {
                let new_start = map.map(val.start);
                new_start..val.len()
            })
            .collect();
    }

    println!("{}", vals.iter().map(|r| r.start).min().unwrap());

    Ok(())
}
