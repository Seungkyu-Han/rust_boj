fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let char_vector: Vec<char> = input.trim().chars().collect();

    let string_vector: Vec<String> = input.trim().chars().map(|s| s.to_string()).collect();

    let length = char_vector.len();

    let mut flag = true;

    for i in 0..(length / 2) {
        if string_vector[i] != string_vector[length - i - 1] {
            flag = false;
            break;
        }
    }

    if flag {
        println!("1");
    }
    else {
        println!("0");
    }
}