fn main() {
    let mut max_value = -1;
    let mut n: usize = 0;
    let mut m: usize = 0;

    for n_index in 0..9 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input_vec: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        for m_index in 0..9 {
            if max_value < input_vec[m_index] {
                max_value = input_vec[m_index];
                n = n_index + 1;
                m = m_index + 1;
            }
        }
    }

    println!("{max_value}");
    println!("{} {}", n, m);
}
