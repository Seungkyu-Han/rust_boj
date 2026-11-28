use std::cmp::max;
use std::io::stdin;
fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let mut tokens = input.split_whitespace();

    let (n, k): (usize, usize) = (
        tokens.next().unwrap().parse().unwrap(),
        tokens.next().unwrap().parse().unwrap(),
    );

    input.clear();

    stdin().read_line(&mut input).unwrap();

    let temperatures: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut cur_acc: i32 = temperatures[..k].iter().sum();
    let mut cur_max = cur_acc;

    for i in 1..(n - k + 1) {
        let next_acc = cur_acc - temperatures[i - 1] + temperatures[i + k - 1];

        cur_acc = next_acc;

        cur_max = max(cur_acc, cur_max);
    }

    println!("{cur_max}");
}
