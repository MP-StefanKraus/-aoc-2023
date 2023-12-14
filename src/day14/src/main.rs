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
    lines = transpose(lines);
    let mut result = 0;
    for l in lines.iter() {
        let moved = move_rocks(l);
        let calced = calculated_weight_on_line(moved);
        result += calced;
    }

    println!("{result}");
}
