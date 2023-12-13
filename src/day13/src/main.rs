use std::cmp::min;
use std::collections::HashSet;
use std::io;

fn transpose(mat: &Vec<String>) -> Vec<String> {
    let res = (0..mat[0].len())
        .map(|i| {
            mat.iter()
                .map(|inner| inner.chars().nth(i).unwrap().clone())
                .collect::<String>()
        })
        .collect();

    res
}

fn get_mirror_points(mat: &Vec<String>) -> HashSet<usize> {
    let mut res = HashSet::new();
    for i in 1..mat[0].len() {
        let mut is_sim = true;
        for line in mat.iter() {
            let (first, second) = line.split_at(i);
            let (first, mut second) = (first.to_string(), second.to_string());
            let mut first_mir = first.chars().rev().collect::<String>();
            let min_len = min(first_mir.len(), second.len());
            assert!(min_len > 0);
            first_mir = first_mir[..min_len].to_string();
            second = second[..min_len].to_string();

            is_sim &= first_mir == second;
            if !is_sim {
                break;
            }
        }

        if is_sim {
            res.insert(i);
        }
    }

    res
}

fn get_pairs(mat: &Vec<String>) -> HashSet<(usize, usize)> {
    let vertical = get_mirror_points(&mat);
    let tt = transpose(mat);
    let horizontal = get_mirror_points(&tt);

    let mut res = HashSet::new();
    for v in vertical {
        res.insert((v, 0));
    }
    for h in horizontal {
        res.insert((0, h));
    }

    res
}

fn main() {
    let mut res = 0;
    loop {
        let mut mat: Vec<String> = vec![];
        let lines = io::stdin().lines().map(|x| x.unwrap());
        for line in lines {
            if line.is_empty() {
                break;
            }
            mat.push(line);
        }
        if mat.is_empty() {
            break;
        };

        let prev_entries = get_pairs(&mat);
        let prev_entry = *prev_entries.iter().next().unwrap();

        let mut found = false;
        'outer: for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                let mut cur = mat.clone();
                let c = cur[i].chars().nth(j).unwrap();
                let n = if c == '.' { "#" } else { "." };
                cur[i].replace_range(j..j + 1, n);

                let pairs = get_pairs(&cur);

                let new_pairs: Vec<_> = pairs.difference(&prev_entries).into_iter().collect();

                if !new_pairs.is_empty() {
                    assert!(new_pairs.len() == 1);
                    let pair = *new_pairs[0];

                    if pair != prev_entry && pair != (0, 0) {
                        found = true;
                        if pair.0 != 0 && pair.0 != prev_entry.0 {
                            res += pair.0;
                        }
                        if pair.1 != 0 && pair.1 != prev_entry.1 {
                            res += 100 * pair.1;
                        }
                        break 'outer;
                    }
                }
            }
        }
        assert!(found);
    }

    println!("{res}");
}
