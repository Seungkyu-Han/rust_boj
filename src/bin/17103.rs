fn prime_vec(n: usize) -> Vec<bool> {
    let mut prime_vec: Vec<bool> = vec![true; n + 1];

    prime_vec[1] = false;

    for i in 2..=n {
        if prime_vec[i] {
            for ii in (i * 2..=n).step_by(i) {
                prime_vec[ii] = false;
            }
        }
    }

    prime_vec
}

fn predict(n: i32, prime_vector: &Vec<bool>) -> i32{
    let mut result = 0;

    for i in 2..=(n/2) {
        if *prime_vector.get(i as usize).unwrap() && *prime_vector.get((n - i) as usize).unwrap() {
            result += 1;
        }
    }

    result
}

fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let t: i32 = input.trim().parse().unwrap();

    let prime_vector = prime_vec(1_000_000);

    for _ in 0..t {
        let mut n_input = String::new();

        std::io::stdin().read_line(&mut n_input).unwrap();

        let n: i32 = n_input.trim().parse().unwrap();

        println!("{}", predict(n, &prime_vector));
    }
}
