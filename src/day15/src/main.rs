use std::io;

fn calculate(s: &str) -> u32 {
    let mut res = 0;
    for c in s.chars() {
        res += c as u32;
        res *= 17;
        res %= 256;
    }

    res
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(Result::unwrap).collect();
    let mut result = 0;
    for l in lines {
        println!("{l}");
        for code in l.split(',') {
            result += calculate(code);
        }
    }

    println!("{result}");
}
