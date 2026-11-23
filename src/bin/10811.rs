fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let n_m_vector = input.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    input.clear();

    let (n, m) = (n_m_vector[0], n_m_vector[1]);

    let mut vector: Vec<usize> = (0..=n).collect();

    for _ in 0..m {
        std::io::stdin().read_line(&mut input).unwrap();
        let i_j_vector = input.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        input.clear();
        let i = i_j_vector[0];
        let j = i_j_vector[1];

        let cnt = (j - i) / 2 + 1;

        for index in 0..cnt {
            let temp = vector[i + index];
            vector[i + index] = vector[j - index];
            vector[j - index] = temp;
        }
    }

    for i in 1..=n {
        print!("{} ", vector[i]);
    }

}
