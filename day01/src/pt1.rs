use day01::*;

fn main() {
    let s = stdin_lines_u8();

    let total = s.map(|l| {
        let a = l.iter().find(|c| c.is_ascii_digit()).expect("at least one digit per line");
        let b = l.iter().rfind(|c| c.is_ascii_digit()).expect("at least one digit per line");
        ((a - b'0') * 10 + b - b'0') as usize
    }).sum::<usize>();

    println!("{total}");
}
