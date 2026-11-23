fn main() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let mut iter = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

        let (a, b) = (iter.next().unwrap(), iter.next().unwrap());

        if a == 0 && b == 0 {
            break;
        }

        println!("{}", a + b);
    }
}