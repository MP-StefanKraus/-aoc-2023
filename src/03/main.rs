use regex::Match;
use regex::Regex;
use std::cmp;
use std::collections::HashSet;
use std::io;

fn get_number_match_sets(field: Vec<String>) -> HashSet<(i32, i32, i32, String)> {
    let pure_numbers = Regex::new(r"([0-9]+)").unwrap();
    let mut matches: HashSet<(i32, i32, i32, String)> = HashSet::new();
    for (i, row) in field.into_iter().enumerate() {
        for num_string in pure_numbers.captures_iter(&row) {
            let f = num_string.get(1).unwrap();
            let start = f.start() as i32;
            let end = f.end() as i32;
            let string = f.as_str();
            matches.insert((i as i32, start, end, string.to_string()));
        }
    }
    matches
}

fn get_symbols(field: Vec<String>) -> HashSet<(i32, i32)> {
    let symbols = Regex::new(r"([^0-9.])").unwrap();
    let mut matches: HashSet<(i32, i32)> = HashSet::new();

    for (i, row) in field.into_iter().enumerate() {
        for sym in symbols.captures_iter(&row) {
            let s = sym.get(1).unwrap().start() as i32;
            matches.insert((i as i32, s));
        }
    }

    matches
}

fn replace_around(field: Vec<String>, symbols: HashSet<(i32, i32)>) -> Vec<String> {
    let mut modified_field = field.clone();
    for (row, col) in symbols {
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                let x = cmp::min(cmp::max(0, row + dx), (field.len() - 1) as i32) as usize;
                let y = cmp::min(cmp::max(0, col + dy), (field[0].len() - 1) as i32) as usize;

                modified_field[x].replace_range(y..y + 1, ".");
            }
        }
    }

    modified_field
}

fn part1() {
    let field: Vec<String> = io::stdin().lines().map(|x| x.unwrap()).collect();

    let nums_before = get_number_match_sets(field.clone());
    let syms = get_symbols(field.clone());

    let modified_field = replace_around(field.clone(), syms);
    let nums_after = get_number_match_sets(modified_field);

    let modified_nums = nums_before.difference(&nums_after);
    let mut sum = 0;
    for (_, _, _, number) in modified_nums {
        sum += number.parse::<i32>().unwrap();
    }

    println!("{sum}");
}

fn main() {
    part1();
}
