fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let mut num = input.trim().parse::<i32>().unwrap();

    while num > 0 {
        print!("long ");
        num -= 4;
    }

    println!("int");
}