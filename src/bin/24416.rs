use std::fmt::Write;

fn main() {
    let mut input = String::new();
    let mut output = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();

    let mut dp: Vec<i32> = vec![0; n + 1];

    dp[1] = 1;
    dp[2] = 1;

    for index in 3..=n {
        dp[index] = dp[index - 1] + dp[index - 2];
    }

    writeln!(output, "{} {}", dp[n], n - 2).unwrap();

    print!("{}", output);

}