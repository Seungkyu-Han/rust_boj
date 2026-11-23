fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().parse::<i32>().unwrap();
    input.clear();

    std::io::stdin().read_line(&mut input).unwrap();

    let vector: Vec<i32> = input
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();

    let v = input.trim().parse::<i32>().unwrap();

    let mut result = 0;

    for x in vector {
        if x == v {
            result += 1;
        }
    }

    println!("{result}");
}