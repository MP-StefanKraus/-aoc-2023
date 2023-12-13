use std::cmp::min;
use std::io;

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

fn get_mirror_point(mat: &Vec<String>) -> Option<usize> {
    for i in 1..mat[0].len() {
        let mut is_sim = true;
        for line in mat.iter() {
            let (first, second) = line.split_at(i);
            let (first , mut second ) = (first.to_string(), second.to_string());
            let mut first_mir = first.chars().rev().collect::<String>();
            let min_len = min(first_mir.len(), second.len());
            assert!(min_len > 0);
            first_mir = first_mir[..min_len].to_string();
            second = second[..min_len].to_string();

            is_sim &= first_mir == second;
            if !is_sim {break}
        }

        if is_sim { return Some(i)}
    }

    None
}

fn main() {

    let mut res = 0;
    loop {
        let mut mat: Vec<String> = vec![];
        let lines = io::stdin().lines().map(|x| x.unwrap());
        for line in lines {
            if line.is_empty() { break; }
            mat.push(line);
        }
        if mat.is_empty() {break};

        let vertical = get_mirror_point(&mat).unwrap_or_default();
        mat = transpose(mat);
        let horizontal = get_mirror_point(&mat).unwrap_or_default();
        assert_ne!(vertical, horizontal);

        res += horizontal * 100 + vertical;
    }

    println!("{res}");
}
