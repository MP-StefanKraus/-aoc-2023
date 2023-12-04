use std::collections::HashSet;
use std::convert::TryInto;
use std::io;
use std::iter::FromIterator;

fn main() {
    let mut sum = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();

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

        if number_of_wins != 0 {
            let base: i64 = 2;
            sum += base
                .checked_pow((number_of_wins - 1).try_into().unwrap())
                .unwrap()
        }
    }

    println!("{sum}");
}
