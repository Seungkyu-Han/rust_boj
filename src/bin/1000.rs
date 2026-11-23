use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let split_whitespace = input.split_whitespace();

    let mut result = 0;

    split_whitespace.for_each(|s| result += s.parse::<i32>().unwrap());

    println!("{}", result);
}