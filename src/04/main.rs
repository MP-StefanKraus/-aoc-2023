use std::cmp;
use std::collections::HashSet;
use std::convert::TryInto;
use std::io;
use std::iter::FromIterator;

fn main() {
    let lines: Vec<String> = io::stdin().lines().map(|x| x.unwrap()).collect();
    let mut cards: Vec<_> = lines.iter().map(|x| 1).collect();

    let mut sum = 0;

    for (i, line) in lines.iter().enumerate() {
        sum += cards[i];

        let (_, numbers) = line.split_once(":").unwrap();
        let (winning_numb_part, my_numbs_part) = numbers.split_once("|").unwrap();

        let winning_numbs: HashSet<i32> = HashSet::from_iter(
            winning_numb_part
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap()),
        );
        let my_numbs: HashSet<i32> = HashSet::from_iter(
            my_numbs_part
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap()),
        );

        let in_both = winning_numbs.intersection(&my_numbs);

        let number_of_wins = in_both.count();

        for j in i + 1..cmp::min(i + number_of_wins + 1, cards.len()) {
            cards[j] += cards[i];
        }
    }

    println!("{sum}");
}
