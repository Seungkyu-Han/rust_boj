fn main() {

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let mut tokens = input.split_whitespace();

    let n = tokens.next().unwrap().parse::<u32>().unwrap();

    let pow = 2_i32.pow(n) + 1;

    println!("{}", pow.pow(2));
}