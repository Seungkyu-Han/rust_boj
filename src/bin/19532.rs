fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let mut tokens = input.split_whitespace();

    let (a, b, c, d, e, f): (i32, i32, i32, i32, i32, i32) = (
        tokens.next().unwrap().parse().unwrap(),
        tokens.next().unwrap().parse().unwrap(),
        tokens.next().unwrap().parse().unwrap(),
        tokens.next().unwrap().parse().unwrap(),
        tokens.next().unwrap().parse().unwrap(),
        tokens.next().unwrap().parse().unwrap(),
    );

    let (mut result_x, mut result_y) = (-1000, -1000);

    for x in -999..1000{
        for y in -999..1000 {
            if (a * x + b * y == c) && (d * x + e * y == f) {
                result_x = x;
                result_y = y;
                break;
            }
        }
    }

    println!("{result_x} {result_y}");
}
