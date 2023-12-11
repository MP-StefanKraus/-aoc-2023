use std::io;

fn expand_horizontal(mat: Vec<String>) -> Vec<String> {
    let mut galaxy_tmp = vec![];
    for line in mat {
        galaxy_tmp.push(line.clone());
        if None == line.find('#') {
            galaxy_tmp.push(line);
        }
    }

    galaxy_tmp
}

fn transpose(mat: Vec<String>) -> Vec<String> {
    let res = (0..mat[0].len())
        .map(|i| mat.iter().map(|inner| inner.chars().nth(i).unwrap().clone()).collect::<String>())
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
    };

    res
}

fn calculate_pairwise_distance(galaxies: Vec<(usize, usize)>) -> usize {
    let mut res = 0;

    for g1 in galaxies.iter() {
        for g2 in galaxies.iter() {
            res += (g2.1).abs_diff(g1.1) + (g2.0).abs_diff(g1.0);
        }
    }

    res /= 2;

    res
}

fn main() {
    let mut lines: Vec<_> = io::stdin().lines().map(|x| x.unwrap()).collect();

    lines = expand_horizontal(lines);
    lines = transpose(lines);
    lines = expand_horizontal(lines);
    lines = transpose(lines);

    let galaxies = find_galaxies(lines);
    let res = calculate_pairwise_distance(galaxies);

    println!("{res}")
}
