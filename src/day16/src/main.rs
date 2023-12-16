use std::collections::HashSet;
use std::io;

fn main() {
    let field: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>()
        .into_iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let mut visited = field
        .iter()
        .map(|x| x.iter().map(|y| HashSet::new()).collect())
        .collect::<Vec<Vec<HashSet<(isize, isize)>>>>();

    let cur: (isize, isize) = (0, 0);

    let mut traces = Vec::new();
    traces.push((cur, (0, 1)));

    let empty_vec = Vec::new();
    let mut iters = 0;
    while !traces.is_empty() && iters < 1000000 {
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
    println!("{res}");
}
