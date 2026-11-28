use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut n_input = String::new();
    stdin().read_line(&mut n_input).unwrap();
    let n: i32 = n_input.trim().parse().unwrap();

    let stdout = stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut stack: Vec<i32> = Vec::with_capacity(1_000_000);

    for _ in 0..n {
        n_input.clear();
        stdin().read_line(&mut n_input).unwrap();

        let mut tokens = n_input.split_whitespace();
        let command: i32 = tokens.next().unwrap().parse().unwrap();

        match command {
            1 => {
                let x = tokens.next().unwrap().parse().unwrap();
                stack.push(x);
            }
            2 => {
                writeln!(out, "{}", stack.pop().unwrap_or(-1)).unwrap();
            }
            3 => {
                writeln!(out, "{}", stack.len()).unwrap();
            }
            4 => {
                writeln!(out, "{}", if stack.is_empty() { 1 } else { 0 }).unwrap();
            }
            5 => {
                writeln!(out, "{}", stack.last().copied().unwrap_or(-1)).unwrap();
            }
            _ => panic!(),
        }
    }
}