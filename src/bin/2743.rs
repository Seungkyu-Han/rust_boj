fn main() {
    let mut input = String::new();

    let result = std::io::stdin().read_line(&mut input).unwrap();

    println!("{}", result - 1);
}