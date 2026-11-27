use std::cmp::max;
use std::fmt::Write;
fn f_gcd(a: i32, b: i32) -> i32 {
    let (_a, _b) = (max(a, b), a + b - max(a, b));

    if _a % _b == 0 { _b } else { f_gcd(_b, _a % _b) }
}

fn main() {
    let mut input = String::new();
    let mut output = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();

    input.clear();

    for _ in 0..n {
        std::io::stdin().read_line(&mut input).unwrap();
    }

    let tokens = input.split_whitespace();

    let mut trees: Vec<i32> = tokens.map(|x| x.parse().unwrap()).collect();

    trees.sort();

    let mut distances: Vec<i32> = vec![0; trees.len() - 1];

    for i in 0..trees.len() - 1 {
        distances[i] = trees[i + 1] - trees[i];
    }

    let mut gcd = distances[0];

    for i in 1..distances.len() {
        gcd = f_gcd(gcd, distances[i]);
    }

    let mut result = 0;

    for distance in distances {
        result += (distance / gcd) - 1;
    }

    writeln!(output, "{result}").unwrap();

    print!("{output}");
}
