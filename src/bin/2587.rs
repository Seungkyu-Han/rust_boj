use std::fmt::Write;

fn main() {
    let mut input = String::new();
    let mut output = String::new();

    std::io::stdin().read_line(&mut input).unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut tokens: Vec<i32> = input.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();

    tokens.sort();

    writeln!(output, "{}", tokens.iter().sum::<i32>() / 5).unwrap();
    writeln!(output, "{}", tokens[2]).unwrap();

    println!("{output}");
}