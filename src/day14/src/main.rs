use std::collections::HashMap;
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

fn move_rocks(s: &String) -> String {
    let mut res: String = s.clone();

    loop {
        let mut switched = false;
        for i in 0..s.len() - 1 {
            let p = res.chars().nth(i).unwrap();
            let q = res.chars().nth(i + 1).unwrap();
            if p == '.' && q == 'O' {
                switched = true;
                res.replace_range(i..(i + 2), format!("{q}{p}").as_str());
            }
        }

        if !switched {
            break;
        }
    }

    res
}

fn calculated_weight_on_line(s: String) -> usize {
    s.chars()
        .enumerate()
        .map(|(i, c)| if c == 'O' { s.len() - i } else { 0 })
        .sum()
}

fn main() {
    let mut lines: Vec<_> = io::stdin().lines().map(Result::unwrap).collect();

    let mut memoiziation: HashMap<Vec<String>, usize> = HashMap::new();

    let mut i = 0;
    let end = 1_000_000_000;
    while i < end {
        //println!("{i}: ");
        //println!("{lines:#?}");
        if memoiziation.contains_key(&*lines) {
            let first_time = memoiziation.get(&*lines).unwrap();
            let diff = i - first_time;
            let steps = (end - i) / diff;
            //println!("{first_time}, {diff}, {steps}, {i}");
            i += diff * steps;
            //println!("{i}");
        }

        memoiziation.insert(lines.clone(), i);
        i += 1;

        // north
        lines = transpose(lines);
        lines = lines.iter().map(move_rocks).collect();

        // west
        lines = transpose(lines);
        lines = lines.iter().map(move_rocks).collect();

        // south
        lines = lines.into_iter().rev().collect();
        lines = transpose(lines);
        lines = lines.iter().map(move_rocks).collect();
        lines = transpose(lines);
        lines = lines.into_iter().rev().collect();

        // east
        lines = transpose(lines);
        lines = lines.into_iter().rev().collect();
        lines = transpose(lines);
        lines = lines.iter().map(move_rocks).collect();
        lines = transpose(lines);
        lines = lines.into_iter().rev().collect();
        lines = transpose(lines);

        let result: usize = transpose(lines.clone())
            .into_iter()
            .map(calculated_weight_on_line)
            .sum();
        println!("{result}");
    }

    lines = transpose(lines);
    println!("{lines:#?}");

    let result: usize = lines.into_iter().map(calculated_weight_on_line).sum();
    println!("{result}");
}
