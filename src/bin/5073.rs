fn main() {
    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();

        let mut tokens = input.split_whitespace();

        let (a, b, c) = (
            tokens.next().unwrap().parse::<i32>().unwrap(),
            tokens.next().unwrap().parse::<i32>().unwrap(),
            tokens.next().unwrap().parse::<i32>().unwrap(),
        );

        if a == 0 && b == 0 && c == 0 {
            return;
        }

        if a == b && b == c {
            println!("Equilateral");
        } else if a + b <= c || a + c <= b || b + c <= a {
            println!("Invalid");
        } else if a == b || b == c || a == c {
            println!("Isosceles");
        } else {
            println!("Scalene");
        }
    }
}
