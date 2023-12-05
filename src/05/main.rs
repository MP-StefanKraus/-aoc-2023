use std::io;

fn simulate(transformations: &Vec<Vec<(i64, i64, i64)>>, value: i64) -> i64 {
    let mut v = value;
    for transform_area in transformations {
        for rule in transform_area {
            let to = rule.0;
            let from = rule.1;
            let length = rule.2;

             if v >= from && v < from + length {
                 let diff = v - from;
                 v = to + diff;
                break;
             }
        }
    }

    v
}

fn main() {

    let mut seedline = String::new();

    let _ = io::stdin().read_line(&mut seedline);

    let seed_str = seedline.split_once(":").unwrap().1;
    let seeds: Vec<_> = seed_str.trim().split(" ").map(|x| x.parse::<i64>().unwrap()).collect();

    let mut transformations: Vec<Vec<(i64, i64, i64)>> = vec![];

    let mut area_transformation: Vec<(i64, i64, i64)> = vec![];

    for line in io::stdin().lines() {
        let line = line.unwrap().trim().to_string();
        if line.is_empty() || line.contains("map") {
            if !area_transformation.is_empty() {
                transformations.push(area_transformation.clone());
                area_transformation.clear();
            }
            continue;
        }

        let almanac_line: Vec<_> = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();

        let to = almanac_line[0];
        let from = almanac_line[1];
        let length = almanac_line[2];

        area_transformation.push((to, from, length));
    }

    let mut min = 1_000_000_000;

    for i in (0..seeds.len()).step_by(2) {
        let start = seeds[i];
        let length = seeds[i+1];

        for j in start..start+length {
            let res = simulate(&transformations, j);
            if res < min {
                min = res;
            }
        }
    }

    println!("{min}");
}
