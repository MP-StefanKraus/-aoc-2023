use std::cmp::max;
use std::collections::HashSet;
use std::io;

fn simulate(start_position: (isize, isize), dir: (isize, isize), field: &Vec<Vec<char>>) -> usize {

    let mut visited = field
        .iter()
        .map(|x| x.iter().map(|y| HashSet::new()).collect())
        .collect::<Vec<Vec<HashSet<(isize, isize)>>>>();

    let mut traces = Vec::new();
    traces.push((start_position, dir));

    let empty_vec = Vec::new();
    while !traces.is_empty() {
        let (pos, dir) = traces.pop().unwrap();

        let val = field
            .get(pos.0 as usize)
            .unwrap_or(&empty_vec)
            .get(pos.1 as usize);
        match val {
            None => (),
            Some(x) => {
                if !visited[pos.0 as usize][pos.1 as usize].contains(&dir) {
                    visited[pos.0 as usize][pos.1 as usize].insert(dir);
                    match x {
                        &'.' => {
                            let np = (pos.0 + dir.0, pos.1 + dir.1);
                            traces.push((np, dir));
                        }
                        &'|' => {
                            if dir == (0, 1) || dir == (0, -1) {
                                let np1 = (pos.0 + 1, pos.1);
                                let np2 = (pos.0 - 1, pos.1);
                                traces.push((np1, (1, 0)));
                                traces.push((np2, (-1, 0)));
                            } else {
                                let np = (pos.0 + dir.0, pos.1 + dir.1);
                                traces.push((np, dir));
                            }
                        }
                        &'-' => {
                            if dir == (1, 0) || dir == (-1, 0) {
                                let np1 = (pos.0, pos.1 + 1);
                                let np2 = (pos.0, pos.1 - 1);
                                traces.push((np1, (0, 1)));
                                traces.push((np2, (0, -1)));
                            } else {
                                let np = (pos.0 + dir.0, pos.1 + dir.1);
                                traces.push((np, dir));
                            }
                        }
                        &'/' => match dir {
                            (0, 1) => {
                                let np = (pos.0 - 1, pos.1);
                                traces.push((np, (-1, 0)));
                            }
                            (0, -1) => {
                                let np = (pos.0 + 1, pos.1);
                                traces.push((np, (1, 0)));
                            }
                            (1, 0) => {
                                let np = (pos.0, pos.1 - 1);
                                traces.push((np, (0, -1)));
                            }
                            (-1, 0) => {
                                let np = (pos.0, pos.1 + 1);
                                traces.push((np, (0, 1)));
                            }
                            _ => panic!(),
                        },
                        &'\\' => match dir {
                            (0, 1) => {
                                let np = (pos.0 + 1, pos.1);
                                traces.push((np, (1, 0)));
                            }
                            (0, -1) => {
                                let np = (pos.0 - 1, pos.1);
                                traces.push((np, (-1, 0)));
                            }
                            (1, 0) => {
                                let np = (pos.0, pos.1 + 1);
                                traces.push((np, (0, 1)));
                            }
                            (-1, 0) => {
                                let np = (pos.0, pos.1 - 1);
                                traces.push((np, (0, -1)));
                            }
                            _ => panic!(),
                        },
                        _ => panic!(),
                    }
                };
            }
        };
    }

    let res: usize = visited
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| if !y.is_empty() { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum();

    res
}

fn main() {
    let field: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>()
        .into_iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();


    let mut res = 0;
    for i in 0..field.len() {
        res = max(res, simulate((i as isize, 0), (0, 1), &field));
        res = max(res, simulate((i as isize, (field[0].len() - 1) as isize), (0, -1), &field));
    }

    for i in 0..field[0].len() {
        res = max(res, simulate((0, i as isize), (1, 0), &field));
        res = max(res, simulate((field.len() as isize -1, i as isize), (-1, 0), &field));
    }

    println!("{res}");
}
