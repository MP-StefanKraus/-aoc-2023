use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;
use std::io;

fn to_num_of_steps(
    start: String,
    commands: &String,
    ways: &HashMap<String, (String, String)>,
) -> u64 {
    let mut cur = start;
    let mut res = 0;
    loop {
        let step = commands
            .chars()
            .nth((res as usize) % commands.len())
            .unwrap();
        match step {
            'L' => cur = ways[&cur].0.clone(),
            'R' => cur = ways[&cur].1.clone(),
            _ => (),
        }

        res += 1;

        if cur.ends_with("Z") {
            return res;
        }
    }
}

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

    let mut steps_nodes: Vec<_> = dirmap
        .clone()
        .into_iter()
        .filter(|(k, v)| k.ends_with("A"))
        .map(|(k, v)| to_num_of_steps(k.to_string(), &commands, &dirmap))
        .collect();

    let res = steps_nodes.iter().fold(1, |a, b| lcm(a, *b));

    println!("{res}");
}
