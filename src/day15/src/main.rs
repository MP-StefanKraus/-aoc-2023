use regex::Regex;
use std::collections::HashMap;
use std::io;

fn calculate(s: &str) -> u32 {
    let mut res = 0;
    for c in s.chars() {
        res += c as u32;
        res *= 17;
        res %= 256;
    }

    res
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(Result::unwrap).collect();

    let re = Regex::new(r"(\w+)([=-])(\d*)$").unwrap();
    let mut hashmap = HashMap::<u32, Vec<(String, u32)>>::new();
    for l in lines {
        println!("{l}");
        for code in l.split(',') {
            for (_, [label, sign, num]) in re.captures_iter(code).map(|c| c.extract()) {
                let boxnum = calculate(label);

                let label = label.to_string();

                let lst = hashmap.entry(boxnum).or_insert(Vec::new());
                let fnd = lst.iter().position(|p| (*p).0 == label);
                match sign {
                    "-" => {
                        match fnd {
                            Some(x) => {
                                lst.remove(x);
                            }
                            None => (),
                        };
                    }
                    "=" => {
                        let num = num.parse().unwrap();
                        match fnd {
                            Some(x) => {
                                lst.remove(x);
                                lst.insert(x, (label, num));
                            }
                            None => {
                                lst.push((label, num));
                            }
                        };
                    }
                    _ => panic!(),
                };
            }
        }
    }

    let mut res = 0;
    for (k, lst) in hashmap.iter() {
        for (i, l) in lst.iter().enumerate() {
            res += (k + 1) * (i as u32 + 1) * l.1;
        }
    }

    println!("{res}");
}
