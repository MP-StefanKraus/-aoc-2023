use std::collections::HashMap;
use std::io;

fn determine_handtype(s: &str) -> i32 {
    let map = s.to_lowercase().chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    let mut values: Vec<_> = map.values().collect();
    values.sort();

    match values.len() {
        1 => return 0,
        2 => match values.get(0).unwrap() {
            1 => return 1,
            2 => return 2,
            _ => todo!(),
        },
        3 => match values.get(2).unwrap() {
            3 => return 3,
            2 => return 4,
            _ => todo!(),
        },
        4 => return 5,
        5 => return 6,
        _ => todo!(),
    }
}

fn get_card_values(s: &str) -> Vec<i32> {
    let vals = "AKQJT98765432";
    return s.chars().map(|x| vals.find(x).unwrap() as i32).collect();
}

fn main() {
    let mut sort_lines = vec![];

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let (hand, point_part) = line.split_once(' ').unwrap();
        let points = point_part.parse::<i32>().unwrap();

        sort_lines.push((determine_handtype(hand), get_card_values(hand), points));
    }
    println!("{sort_lines:?}");

    sort_lines.sort();

    let mut sum = 0;
    for (i, val) in sort_lines.iter().rev().enumerate() {
        sum += val.2 * (i as i32 + 1);
    }
    println!("{sum}");
}
