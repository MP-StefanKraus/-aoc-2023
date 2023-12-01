use std::io;

fn main() {
    let converting = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let mut sum = 0;
    for input in io::stdin().lines() {
        let line = input.unwrap();
        let mut pos_numbers = Vec::new();
        for case in converting {
            let (phrase, num) = case;
            let first_pos = line.find(phrase);
            let last_pos = line.rfind(phrase);
            match first_pos {
                Some(t) => pos_numbers.push((t, num)),
                None => (),
            }
            match last_pos {
                Some(t) => pos_numbers.push((t, num)),
                None => (),
            }

        }
        pos_numbers.sort();

        let first_digit = pos_numbers[0].1;
        let last_digit = pos_numbers[pos_numbers.len()-1].1;
        let number = first_digit * 10 + last_digit;
        sum += number;
    }
    println!("{}", sum);
}
