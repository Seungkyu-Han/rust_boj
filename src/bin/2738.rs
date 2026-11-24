fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let n_m_vector: Vec<usize> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let n = n_m_vector[0];
    let m = n_m_vector[1];

    input.clear();

    let mut vector: Vec<Vec<i32>> = vec![vec![0i32; m]; n];

    for _ in 0..2 {
        for n_index in 0..n {
            std::io::stdin().read_line(&mut input).unwrap();
            let row: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
            input.clear();

            for m_index in 0..m {
                vector[n_index][m_index] += row[m_index];
            }
        }
    }

    for n_index in 0..n {
        for m_index in 0..m {
            print!("{} ", vector[n_index][m_index])
        }
        println!();
    }
}