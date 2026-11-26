use std::cmp::max;

fn gcd(a: i32, b: i32) -> i32 {
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
    std::io::stdin().read_line(&mut input).unwrap();

    let mut tokens = input.split_whitespace();

    let (a, b, c, d): (i32, i32, i32, i32) = (
        tokens.next().unwrap().parse().unwrap(),
        tokens.next().unwrap().parse().unwrap(),
        tokens.next().unwrap().parse().unwrap(),
        tokens.next().unwrap().parse().unwrap(),
    );

    let num = a * d + c * b;
    let den = b * d;

    let common = gcd(num, den);

    println!("{} {}", num / common, den / common);
}
