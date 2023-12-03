use day03::*;

fn main() -> anyhow::Result<()> {
    let grid = stdin_lines_u8().collect_vec();

    println!(
        "{}",
        find_part_numbers(&grid)
            .into_iter()
            .map(|pn| pn.1)
            .sum::<usize>()
    );
    Ok(())
}
