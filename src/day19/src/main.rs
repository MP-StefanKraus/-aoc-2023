use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Rule {
    target_yes: String,
    expr: Option<String>,
    target_no: Option<String>,
}

#[derive(Debug, Clone)]
struct InputValues {
    minmaxs: [(u64, u64); 4],
}

fn parse_input() -> HashMap<String, Rule> {
    let lines: Vec<_> = io::stdin().lines().map(|x| x.unwrap()).collect();

    let re_rules = Regex::new(r"(\w+)\{(.+)\}").unwrap();

    let mut rules: HashMap<String, Rule> = HashMap::new();

    for l in lines {
        for (_, [from, decisions]) in re_rules.captures_iter(l.as_str()).map(|c| c.extract()) {
            let from = from.to_string();
            for (i, decision) in decisions.split(",").enumerate() {
                let indexed_from = format!("{from}_{i}");
                let nxt = i + 1;
                let target_no = format!("{from}_{nxt}");
                let rule;
                if decision.contains(":") {
                    let (expr, target_yes) = decision.split_once(":").unwrap();
                    let target_yes = format!("{target_yes}_0");
                    rule = Rule {
                        target_yes,
                        expr: Some(expr.to_string()),
                        target_no: Some(target_no.to_string()),
                    };
                } else {
                    rule = Rule {
                        target_yes: format!("{decision}_0"),
                        expr: None,
                        target_no: None,
                    };
                }
                rules.insert(indexed_from.clone(), rule);
            }
        }
    }

    rules
}

fn count_accept(input: InputValues, rules: &HashMap<String, Rule>, cur: String) -> u64 {
    if input.minmaxs.iter().any(|x| x.1 < x.0) {
        return 0;
    }

    if cur == String::from("A_0") {
        return input
            .minmaxs
            .iter()
            .map(|x| x.1 - x.0 + 1)
            .reduce(|a, b| a * b)
            .unwrap();
    }
    if cur == String::from("R_0") {
        return 0;
    }

    let order = "xmas";

    let rule = rules.get(&cur).unwrap();
    match &rule.expr {
        None => {
            return count_accept(input, rules, rule.target_yes.clone());
        }
        Some(xp) => {
            let mut characters = xp.chars();
            let ch = characters.next().unwrap();
            let sign = characters.next().unwrap();
            let border: u64 = characters.as_str().to_string().parse().unwrap();

            let allowed;
            let denied;
            if sign == '<' {
                allowed = (1, border - 1);
                denied = (border, 4000);
            } else if sign == '>' {
                allowed = (border + 1, 4000);
                denied = (1, border);
            } else {
                panic!()
            }

            let pos = order.find(ch).unwrap();

            let mut accepting_input = input.clone();
            accepting_input.minmaxs[pos] = (
                max(accepting_input.minmaxs[pos].0, allowed.0),
                min(accepting_input.minmaxs[pos].1, allowed.1),
            );
            let mut denying_input = input.clone();
            denying_input.minmaxs[pos] = (
                max(denying_input.minmaxs[pos].0, denied.0),
                min(denying_input.minmaxs[pos].1, denied.1),
            );

            return count_accept(accepting_input, rules, rule.target_yes.clone())
                + count_accept(
                    denying_input,
                    rules,
                    rule.target_no.clone().unwrap().clone(),
                );
        }
    }
}

fn main() {
    let rules = parse_input();

    println!("{rules:#?}");

    let input = InputValues {
        minmaxs: [(1, 4000), (1, 4000), (1, 4000), (1, 4000)],
    };
    let result = count_accept(input, &rules, String::from("in_0"));
    println!("{result}");
}
