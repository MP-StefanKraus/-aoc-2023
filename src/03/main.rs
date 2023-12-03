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

fn search_numbers(
    field: Vec<String>,
    symbols: HashSet<(i32, i32)>,
    numbers: HashSet<(i32, i32, i32, String)>,
) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    for s in symbols.iter() {
        let row = s.0 as usize;
        let col = s.1 as usize;
        let symbol = field[row].chars().nth(col).unwrap();

        if symbol != '*' {
            continue;
        }

        let mut symbol_rel_pos: HashSet<(usize, usize)> = HashSet::new();

        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                let x = cmp::min(cmp::max(0, row as i32 + dx), (field.len() - 1) as i32) as usize;
                let y =
                    cmp::min(cmp::max(0, col as i32 + dy), (field[0].len() - 1) as i32) as usize;

                symbol_rel_pos.insert((x, y));
            }
        }

        let mut adjacent_numbers: Vec<i32> = Vec::new();

        for num_match in &numbers {
            for i in num_match.1..num_match.2 {
                if symbol_rel_pos.contains(&(num_match.0 as usize, i as usize)) {
                    adjacent_numbers.push(num_match.3.parse().unwrap());
                    break;
                }
            }
        }

        if adjacent_numbers.len() == 2 {
            let mut it = adjacent_numbers.iter();
            result.push((*it.next().unwrap(), *it.next().unwrap()));
        }
    }

    result
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

fn part2() {
    let field: Vec<String> = io::stdin().lines().map(|x| x.unwrap()).collect();

    let nums_before = get_number_match_sets(field.clone());
    let syms = get_symbols(field.clone());

    let numbers = search_numbers(field, syms, nums_before);
    let mut sum = 0;

    for (n1, n2) in numbers.into_iter() {
        sum += n1 * n2;
    }
    println!("{sum}");
}

fn main() {
    part2();
}
