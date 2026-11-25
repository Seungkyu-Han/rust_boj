fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let k: usize = tokens.next().unwrap().parse().unwrap();

    let mut share_vec: Vec<usize> = Vec::with_capacity(n);

    for index in 1..=n {
        if n % index == 0 {
            share_vec.push(index);
        }
    }

    println!("{}", if k > share_vec.len() {0} else {share_vec[k - 1]})
}
