fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();

    for index in 0..n {
        for _ in 0..(n - index - 1) {
            print!(" ");
        }

        for _ in 0..(index * 2 + 1) {
            print!("*");
        }

        println!();
    }

    for index in (0..n - 1).rev() {
        for _ in 0..(n - index - 1) {
            print!(" ");
        }

        for _ in 0..(index * 2 + 1) {
            print!("*");
        }

        if index != 0 {
            println!();
        }

    }
}
