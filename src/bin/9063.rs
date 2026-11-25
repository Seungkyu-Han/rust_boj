use std::cmp::{max, min};
use std::fmt::Write;
fn main() {
    let mut input = String::new();
    let mut output = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();

    input.clear();

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for _ in 0..n {
        std::io::stdin().read_line(&mut input).unwrap();

        let mut tokens = input.split_whitespace();

        let x: i32 = tokens.next().unwrap().parse().unwrap();
        let y: i32 = tokens.next().unwrap().parse().unwrap();

        input.clear();

        min_x = min(min_x, x);
        max_x = max(max_x, x);

        min_y = min(min_y, y);
        max_y = max(max_y, y);
    }

    write!(output, "{}", (max_x-min_x) * (max_y - min_y)).unwrap();

    print!("{output}");
}