use std::io;

fn main() {
    let mut sum = 0;
    for input in io::stdin().lines() {
        let line = input.unwrap();
        let a: Vec<_> = line.chars().filter(|x| x.is_digit(10)).collect();
        let first_digit = a[0].to_digit(10).unwrap();
        let last_digit = a[a.len()-1].to_digit(10).unwrap();
        let number = first_digit * 10 + last_digit;
        sum += number;
    }
    println!("{}", sum)
}
