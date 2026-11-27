fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut tokens = input.split_whitespace();

    let (n, k): (usize, usize) = (
        tokens.next().unwrap().parse().unwrap(),
        tokens.next().unwrap().parse().unwrap(),
    );

    let mut x: Vec<i32> = tokens.map(|x| x.parse().unwrap()).collect();

    x.sort();

    println!("{}", x[n - k]);
}
