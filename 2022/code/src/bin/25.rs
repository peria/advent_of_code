use std::io::BufRead;

fn main() {
    let numbers = get_input();

    println!("{}", get_snafu_sum(&numbers));
}

fn get_snafu_sum(numbers: &Vec<String>) -> String {
    let mut sum = 0;
    for snafu in numbers {
        sum += snafu_to_i64(snafu);
    }
    i64_to_snafu(sum)
}

fn snafu_to_i64(snafu: &str) -> i64 {
    let mut num = 0;
    for c in snafu.chars() {
        num = 5 * num
            + match c {
                '=' => -2,
                '-' => -1,
                '1' => 1,
                '2' => 2,
                _ => 0,
            };
    }
    num
}

fn i64_to_snafu(num: i64) -> String {
    let mut snafu = String::new();
    let mut num = num;
    loop {
        if num == 0 {
            break;
        }
        match num % 5 {
            1 => {
                snafu += "1";
                num -= 1;
            }
            2 => {
                snafu += "2";
                num -= 2;
            }
            3 => {
                snafu += "=";
                num += 2;
            }
            4 => {
                snafu += "-";
                num += 1;
            }
            _ => {
                snafu += "0";
            }
        }
        num /= 5;
    }
    snafu.chars().rev().collect()
}

fn get_input() -> Vec<String> {
    let mut numbers = Vec::new();
    for line in std::io::stdin().lock().lines() {
        numbers.push(line.unwrap());
    }
    numbers
}
