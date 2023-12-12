use std::io;

fn generate_possibilities(s: String) -> Vec<String> {

    let mut res = vec![];

    let pos = s.find("?");
    match pos {
        Some(i) => {
            let mut s1 = s.clone();
            let mut s2 = s.clone();
            s1.replace_range(i..i+1, ".");
            s2.replace_range(i..i+1, "#");
            let r1 = generate_possibilities(s1);
            let r2 = generate_possibilities(s2);
            res.extend(r1);
            res.extend(r2);
        },
        None => res.push(s),
    };

    res
}

fn main() {
    let mut res = 0;
    for line in io::stdin().lines() {

        let line = line.unwrap();

        let (fountain_part, number_part) = line.split_once(" ").unwrap();
        let numbers: Vec<i32> = number_part.split(",").map(|x| x.parse().unwrap()).collect();

        let combinations = generate_possibilities(fountain_part.to_string());
        let mut count = 0;
        for c in combinations {
            let test: Vec<_> = c.split('.').map(|x| x.len() as i32).filter(|x| *x != 0).collect();
            //println!("{test:?}");
            if test == numbers {
                count += 1
            }
        }
        //println!("{count}");
        res += count;
    }

    println!("{res}");
}
