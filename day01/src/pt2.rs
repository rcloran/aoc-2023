use day01::*;

fn main() {
    let s = stdin_lines_u8();

    let table = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .into_iter()
    .enumerate()
    .map(|(i, w)| (w.as_bytes().to_vec(), i + 1))
    .chain((0..=9).map(|d| ([d + b'0'].to_vec(), d as usize)))
    .collect_vec();

    let total = s.into_iter().map(|l| {
        let mut digits = vec![];
        for i in 0..l.len() {
            for (k, v) in table.iter() {
                if l[i..].starts_with(k) {
                    digits.push(v);
                    break;
                }
            }
        }
        digits[0] * 10 + digits[digits.len() - 1]
    }).sum::<usize>();

    println!("{total}");
}
