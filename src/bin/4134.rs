fn is_prime(n: u64) -> bool {
    match n {
        ..=1 => false,

        2 | 3 => true,
        x if x % 2 == 0 || x % 3 == 0 => false,

        x => {
            let mut i = 5;
            while i * i <= x {
                if x % i == 0 || x % (i + 2) == 0 {
                    return false;
                }
                i += 6;
            }
            true
        }
    }
}

fn main() {

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let rep_cnt: usize = input.trim().parse().unwrap();

    drop(input);

    for _ in 0..rep_cnt {
        let mut n_input = String::new();

        std::io::stdin().read_line(&mut n_input).unwrap();

        let n: u64 = n_input.trim().parse().unwrap();

        let mut cur_n = n;

        while !is_prime(cur_n) {
            cur_n += 1;
        }

        println!("{cur_n}");
    }
}