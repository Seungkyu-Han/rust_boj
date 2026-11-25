fn main() {
    loop {
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();

        let n: i32 = input.trim().parse().unwrap();

        if n == -1 {
            break;
        }

        let mut cur = 0;

        let mut small_vec: Vec<i32> = Vec::with_capacity(n as usize);
        let mut big_vec: Vec<i32> = Vec::with_capacity(n as usize);

        for index in 1..=n.isqrt() {
            if n % index == 0 {
                if n / index == index {
                    cur += index;

                    small_vec.push(index);
                } else {
                    cur += index;
                    cur += n / index;

                    small_vec.push(index);
                    big_vec.push(n / index);
                }
            }
        }

        if cur == n * 2 {
            big_vec.reverse();

            big_vec.pop();
            let last_value = big_vec.last().copied().unwrap_or(-1);

            print!("{n} = ");

            for i in small_vec {
                print!("{i} + ");
            }

            for i in big_vec {
                print!("{i}");

                if i != last_value {
                    print!(" + ");
                }
            }
            println!();
        } else {
            println!("{n} is NOT perfect.");
        }
    }
}
