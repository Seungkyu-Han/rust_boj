fn main() {
    let mut input = String::new();

    let mut vector: Vec<bool> = (0..31).map(|_value| true).collect();

    for _ in 0..28 {
        std::io::stdin().read_line(&mut input).unwrap();

        let student: usize = input.trim().parse().unwrap();

        input.clear();

        vector[student] = false;
    }

    for i in 1..=30 {
        if vector[i] == true {
            println!("{i}");
        }
    }
}
