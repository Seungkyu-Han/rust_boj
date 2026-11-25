use std::fmt::Write;

fn main() {
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();

    let mut tokens = input.split_whitespace();

    let (n, m) = (
        tokens.next().unwrap().parse::<usize>().unwrap(),
        tokens.next().unwrap().parse::<usize>().unwrap(),
    );

    let mut cur_vec = Vec::with_capacity(m);
    let mut output = String::new();

    back_tracking(&mut cur_vec, n, m, &mut output);

    print!("{output}");
}

fn back_tracking(cur_vec: &mut Vec<i32>, n: usize, m: usize, output: &mut String) {
    if cur_vec.len() == m {
        for value in cur_vec {
            write!(output, "{value} ").unwrap();
        }
        writeln!(output).unwrap();

        return;
    }

    for i in 1..=n {
        cur_vec.push(i as i32);

        back_tracking(cur_vec, n, m, output);

        cur_vec.pop();
    }
}
