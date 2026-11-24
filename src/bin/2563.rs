fn main() {
    let mut color_paper: Vec<Vec<i32>> = vec![vec![0;101]; 101];

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    for _ in 0..input.trim().parse().unwrap() {
        input.clear();

        std::io::stdin().read_line(&mut input).unwrap();

        let x_y_vector: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

        for x in 0..10 {
            for y in 0..10 {
                color_paper[x + x_y_vector[0]][y + x_y_vector[1]] += 1;
            }
        }
    }

    let mut result = 0;

    for x in 1..101 {
        for y in 1..101 {
            if color_paper[x][y] > 0 {
                result += 1;
            }
        }
    }

    println!("{result}");
}