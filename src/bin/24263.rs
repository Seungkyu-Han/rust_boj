use std::fmt::Write;

fn main() {
    let mut input = String::new();
    let mut output = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let mut tokens = input.split_whitespace();

    let n: i32 = tokens.next().unwrap().parse().unwrap();

    writeln!(output, "{n}", ).unwrap();
    writeln!(output, "1", ).unwrap();

    print!("{output}")
}