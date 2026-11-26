use std::cmp::{max, min};

fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let mut tokens = input.split_whitespace();

    let (a, b, c): (i32, i32, i32) = (
        tokens.next().unwrap().parse().unwrap(),
        tokens.next().unwrap().parse().unwrap(),
        tokens.next().unwrap().parse().unwrap(),
    );

    let max_value = max(a, max(b, c));

    let max_length = min((a + b + c) - max_value - 1, max_value);

    println!("{}", a + b + c - max_value + max_length);
}
