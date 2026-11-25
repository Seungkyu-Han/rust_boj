use std::collections::HashMap;

fn main() {
    let mut input = String::new();

    let hex_map: HashMap<char, u64> = HashMap::from([
        ('0', 0),
        ('1', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('A', 10),
        ('B', 11),
        ('C', 12),
        ('D', 13),
        ('E', 14),
        ('F', 15),
        ('G', 16),
        ('H', 17),
        ('I', 18),
        ('J', 19),
        ('K', 20),
        ('L', 21),
        ('M', 22),
        ('N', 23),
        ('O', 24),
        ('P', 25),
        ('Q', 26),
        ('R', 27),
        ('S', 28),
        ('T', 29),
        ('U', 30),
        ('V', 31),
        ('W', 32),
        ('X', 33),
        ('Y', 34),
        ('Z', 35),
    ]);

    std::io::stdin().read_line(&mut input).unwrap();

    let tokens: Vec<&str> = input.split_whitespace().collect();

    let n: Vec<char> = tokens[0].trim().chars().collect();
    let b: u64 = tokens[1].parse().unwrap();

    let mut result: u64 = 0;

    for cur_token in n {

        result = result * b;

        result += hex_map.get(&cur_token).unwrap();
    }

    print!("{result}");
}