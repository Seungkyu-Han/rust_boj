fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let t = input.trim().parse::<i32>().unwrap();

    input.clear();

    for _ in 0..t {
        std::io::stdin().read_line(&mut input).unwrap();
        let first = input.trim().chars().nth(0).unwrap();
        let last = input.trim().chars().last().unwrap();
        println!("{}{}", first, last);
        input.clear();
    }
}