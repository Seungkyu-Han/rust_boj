fn main() {
    let mut string_vec: Vec<Vec<String>> = Vec::new();

    for _ in 0..5 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input_vector = input.trim().chars().map(|x| x.to_string()).collect();

        string_vec.push(input_vector);
    }

    for m_index in 0..15 {
        for n_index in 0..5 {
            if string_vec[n_index].len() > m_index {
                print!("{}", string_vec[n_index][m_index]);
            }
        }
    }
}