use std::collections::HashMap;
use std::io;

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
enum Types {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPairs,
    OnePair,
    HighCard,
}

fn determine_handtype(s: &str) -> Types {
    let mut map = s.to_lowercase().chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1;
        acc
    });

    let mut num_joker = 0;
    if map.contains_key(&'j') {
        num_joker = map[&'j'];
        map.remove(&'j');
    }

    let mut values: Vec<_> = map.values().collect();
    values.sort();

    match num_joker {
        0 => match values.len() {
            1 => return Types::FiveKind,
            2 => match values.get(0).unwrap() {
                1 => return Types::FourKind,
                2 => return Types::FullHouse,
                _ => todo!(),
            },
            3 => match values.get(2).unwrap() {
                3 => return Types::ThreeKind,
                2 => return Types::TwoPairs,
                _ => todo!(),
            },
            4 => return Types::OnePair,
            5 => return Types::HighCard,
            _ => todo!(),
        },
        1 => match values.len() {
            1 => return Types::FiveKind,
            2 => match values.get(1).unwrap() {
                3 => return Types::FourKind,
                2 => return Types::FullHouse,
                _ => todo!(),
            },
            3 => return Types::ThreeKind,
            4 => return Types::OnePair,
            _ => todo!(),
        },
        2 => match values.len() {
            1 => return Types::FiveKind,
            2 => return Types::FourKind,
            3 => return Types::ThreeKind,
            _ => todo!(),
        },
        3 => match values.len() {
            1 => return Types::FiveKind,
            2 => return Types::FourKind,
            _ => todo!(),
        },
        4 | 5 => return Types::FiveKind,
        _ => todo!(),
    }
}

fn get_card_values(s: &str) -> Vec<i32> {
    let vals = "AKQT98765432J";
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

    sort_lines.sort();

    let mut sum = 0;
    for (i, val) in sort_lines.iter().rev().enumerate() {
        sum += val.2 * (i as i32 + 1);
    }
    println!("{sum}");
}
