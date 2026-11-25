use std::fmt::Write;
fn main() {
    let mut input = String::new();
    let mut output = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let a: i32 = input.trim().parse().unwrap();

    input.clear();

    std::io::stdin().read_line(&mut input).unwrap();

    let b: i32 = input.trim().parse().unwrap();

    input.clear();

    write!(output, "{}", a * b).unwrap();

    print!("{output}");
}