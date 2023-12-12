use std::collections::HashMap;
use std::io;

fn get_sum_of_possibilities(
    s: String,
    numbers: Vec<i64>,
    dp: &mut HashMap<(String, Vec<i64>), i64>,
) -> i64 {
    let key = (s.clone(), numbers.clone());

    let mut res = 0;

    if !dp.contains_key(&key) {
        if s.is_empty() {
            res = if numbers.is_empty() { 1 } else { 0 };
        } else {
            let nxt_char = s.chars().nth(0).unwrap();

            if nxt_char == '.' {
                res = get_sum_of_possibilities(s[1..].to_string(), numbers.clone(), dp);
            } else if nxt_char == '?' {
                let mut s1 = s.clone();
                let mut s2 = s.clone();
                s1.replace_range(0..1, ".");
                s2.replace_range(0..1, "#");
                res = get_sum_of_possibilities(s1, numbers.clone(), dp)
                    + get_sum_of_possibilities(s2, numbers.clone(), dp);
            } else if nxt_char == '#' {
                if numbers.is_empty() {
                    res = 0;
                } else {
                    let cur = numbers[0] as usize;
                    if s.len() < cur + 1 {
                        return 0;
                    };

                    let mut chrs = s.chars();

                    for i in 0..cur {
                        let c = chrs.next().unwrap();
                        if c == '.' {
                            return 0;
                        }
                    }
                    if chrs.next().unwrap() == '#' {
                        return 0;
                    }

                    let rest: String = chrs.collect();

                    res = get_sum_of_possibilities(
                        rest,
                        numbers[1..].iter().map(|x| *x).collect(),
                        dp,
                    );
                }
            } else {
                panic!("I should not have seen this char?");
            }
        }
        dp.insert(key.clone(), res);
    }

    return *dp.get(&key).unwrap();
}

fn main() {
    let mut res = 0;
    for line in io::stdin().lines() {
        let line = line.unwrap();

        let (fountain_part, number_part) = line.split_once(" ").unwrap();
        let mut numbers: Vec<i64> = number_part.split(",").map(|x| x.parse().unwrap()).collect();

        let mut fountain = { fountain_part.to_string() + "?" }.repeat(5);
        fountain.replace_range(fountain.len() - 1.., ".");
        numbers = numbers.repeat(5);

        let mut dp = HashMap::new();
        let result = get_sum_of_possibilities(fountain, numbers, &mut dp);
        res += result;
    }

    println!("{res}");
}
