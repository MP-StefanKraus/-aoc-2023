use std::cmp::{max, min};
use std::io;

fn find_rows_without_galaxies(mat: &Vec<String>) -> Vec<usize> {
    let mut rows = vec![];
    for (i, line) in mat.iter().enumerate() {
        if None == line.find('#') {
            rows.push(i);
        }
    }

    rows
}

fn transpose(mat: Vec<String>) -> Vec<String> {
    let res = (0..mat[0].len())
        .map(|i| {
            mat.iter()
                .map(|inner| inner.chars().nth(i).unwrap().clone())
                .collect::<String>()
        })
        .collect();

    res
}

fn find_galaxies(mat: Vec<String>) -> Vec<(usize, usize)> {
    let mut res = vec![];

    for (i, line) in mat.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                res.push((i, j));
            }
        }
    }

    res
}

const EMPTY_VALUE: usize = 1_000_000;

fn calculate_pairwise_distance(
    galaxies: Vec<(usize, usize)>,
    rows: Vec<usize>,
    cols: Vec<usize>,
) -> usize {
    let mut res = 0;

    for g1 in galaxies.iter() {
        for g2 in galaxies.iter() {
            for i in min(g1.0, g2.0)..max(g1.0, g2.0) {
                match rows.iter().find(|x| **x == i) {
                    Some(_x) => res += EMPTY_VALUE,
                    None => res += 1,
                }
            }
            for i in min(g1.1, g2.1)..max(g1.1, g2.1) {
                match cols.iter().find(|x| **x == i) {
                    Some(_x) => res += EMPTY_VALUE,
                    None => res += 1,
                }
            }
        }
    }

    res /= 2;

    res
}

fn main() {
    let mut lines: Vec<_> = io::stdin().lines().map(|x| x.unwrap()).collect();

    let rows = find_rows_without_galaxies(&lines);
    lines = transpose(lines);
    let cols = find_rows_without_galaxies(&lines);
    lines = transpose(lines);

    let galaxies = find_galaxies(lines);

    let res = calculate_pairwise_distance(galaxies, rows, cols);

    println!("{res}")
}
