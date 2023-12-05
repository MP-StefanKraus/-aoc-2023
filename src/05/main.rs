use std::io;

fn main() {

    let mut seedline = String::new();

    let _ = io::stdin().read_line(&mut seedline);

    let seed_str = seedline.split_once(":").unwrap().1;
    let mut seeds: Vec<_> = seed_str.trim().split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
    let mut mutated: Vec<bool> = seeds.iter().map(|_x| false).collect();

    for line in io::stdin().lines() {
        let line = line.unwrap().trim().to_string();
        if line.is_empty() || line.contains("map") {
            mutated = seeds.iter().map(|_x| false).collect();
            continue;
        }

        let almanac_line: Vec<_> = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
        let to = almanac_line[0];
        let from = almanac_line[1];
        let length = almanac_line[2];
        println!("{}", line);

        for (i, seed) in seeds.clone().into_iter().enumerate() {
            if !mutated[i] && seed >= from && seed < from + length {
                let diff = seed - from;
                seeds[i] = to + diff;
                mutated[i] = true;
            }
        }

        println!("{:?}", seeds);
    }
    println!("{}", seeds.iter().min().unwrap())
}
