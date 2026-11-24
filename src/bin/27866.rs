fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    let mut i_string = String::new();
    std::io::stdin().read_line(&mut i_string).unwrap();

    let i: usize = i_string.trim().parse::<usize>().unwrap();

    let result: char = s.chars().nth(i - 1).unwrap();

    println!("{result}");
}
