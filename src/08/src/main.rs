use std::io;
use regex::Regex;
use std::collections::HashMap;


fn main() {

    let stdin = io::stdin();
    let mut stdin_iter = stdin.lines();
    let commands = stdin_iter.next().unwrap().unwrap();
    stdin_iter.next().unwrap().unwrap();

    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let mut dirmap = HashMap::<String, (String, String)>::new();

    for net in stdin_iter {
        let net = net.unwrap();

        for (_, [from, left, right]) in re.captures_iter(&net).map(|x| x.extract()) {
            dirmap.insert(from.to_string(), (left.to_string(), right.to_string()));
        }
    }

    let mut cur = "AAA".to_string();
    let mut res = 0;
    loop {
        let step = commands.chars().nth(res % commands.len()).unwrap();
        match step {
            'L' => cur = dirmap[&cur].0.clone(),
            'R' => cur = dirmap[&cur].1.clone(),
            _ => (),
        }

        res += 1;

        if cur == "ZZZ" {
            break;
        }


    }

    println!("{res}");

}
