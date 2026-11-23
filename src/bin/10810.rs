fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let mut it = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    let (n, m) = (it.next().unwrap(), it.next().unwrap());

    let mut vector: Vec<usize> = (0..=n).map(|_value| 0).collect();

    for _ in 0..m {
        input.clear();

        std::io::stdin().read_line(&mut input).unwrap();

        let mut it = input
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap());

        let (i, j, k) = (it.next().unwrap(), it.next().unwrap(), it.next().unwrap());

        for index in i..=j {
            vector[index] = k;
        }
    }

    for element in &vector[1..] {
        print!("{} ", element);
    }
}
