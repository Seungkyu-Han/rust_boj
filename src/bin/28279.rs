use std::collections::VecDeque;
use std::io::{self, BufRead, BufWriter, Write};

fn main() {

    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let mut input = String::new();
    stdin_lock.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut deque: VecDeque<i32> = VecDeque::with_capacity(n);
    input.clear();

    for _ in 0..n {
        stdin_lock.read_line(&mut input).unwrap();

        let mut iter = input.split_whitespace();
        let cmd: i32 = iter.next().unwrap().parse().unwrap();

        match cmd {
            1 => {
                let x: i32 = iter.next().unwrap().parse().unwrap();
                deque.push_front(x);
            }
            2 => {
                let x: i32 = iter.next().unwrap().parse().unwrap();
                deque.push_back(x);
            }
            3 => {
                writeln!(out, "{}", deque.pop_front().unwrap_or(-1)).unwrap();
            }
            4 => {
                writeln!(out, "{}", deque.pop_back().unwrap_or(-1)).unwrap();
            }
            5 => {
                writeln!(out, "{}", deque.len()).unwrap();
            }
            6 => {
                writeln!(out, "{}", if deque.is_empty() { 1 } else { 0 }).unwrap();
            }
            7 => {
                writeln!(out, "{}", deque.front().unwrap_or(&-1)).unwrap();
            }
            8 => {
                writeln!(out, "{}", deque.back().unwrap_or(&-1)).unwrap();
            }
            _ => panic!(),
        }

        input.clear();
    }
}