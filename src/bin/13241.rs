use std::cmp::max;

fn gcd(a: u64, b: u64) -> u64 {
    let max_value = max(a, b);
    let min_value = a + b - max_value;

    if max_value % min_value == 0 {
        min_value
    } else {
        gcd(min_value, max_value % min_value)
    }
}

fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let mut tokens = input.split_whitespace();

    let (a, b): (u64, u64) = (
        tokens.next().unwrap().parse().unwrap(),
        tokens.next().unwrap().parse().unwrap(),
    );

    println!("{}", (a * b) / gcd(a, b));
}
