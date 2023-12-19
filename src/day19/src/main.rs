use evalexpr::*;
use regex::Regex;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Decision {
    expr: String,
    target: String,
}
#[derive(Debug)]
struct Rule {
    from: String,
    decisions: Vec<Decision>,
}

#[derive(Debug)]
struct InputValues {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

fn parse_input() -> (HashMap<String, Rule>, Vec<InputValues>) {
    let lines: Vec<_> = io::stdin().lines().map(|x| x.unwrap()).collect();

    let re_rules = Regex::new(r"(\w+)\{(.+)\}").unwrap();
    let re_input = Regex::new(r"x=(\d+),m=(\d+),a=(\d+),s=(\d+)").unwrap();

    let mut rules: HashMap<String, Rule> = HashMap::new();
    let mut inputs = vec![];

    for l in lines {
        for (_, [from, decisions]) in re_rules.captures_iter(l.as_str()).map(|c| c.extract()) {
            let from = from.to_string();
            let mut decs = vec![];
            for decision in decisions.split(",") {
                if decision.contains(":") {
                    let (expr, target) = decision.split_once(":").unwrap();
                    let dec = Decision {
                        expr: expr.to_string(),
                        target: target.to_string(),
                    };
                    decs.push(dec);
                } else {
                    let dec = Decision {
                        expr: "0<1".to_string(),
                        target: decision.to_string(),
                    };
                    decs.push(dec);
                }
            }
            rules.insert(
                from.clone(),
                Rule {
                    from,
                    decisions: decs,
                },
            );
        }

        for (_, [x, m, a, s]) in re_input.captures_iter(l.as_str()).map(|c| c.extract()) {
            let x = x.parse().unwrap();
            let m = m.parse().unwrap();
            let a = a.parse().unwrap();
            let s = s.parse().unwrap();

            inputs.push(InputValues { x, m, a, s });
        }
    }

    (rules, inputs)
}

fn shall_accept(input: &InputValues, rules: &HashMap<String, Rule>) -> bool {
    let mut cur = "in".to_string();

    let context = context_map! {
        "x" => input.x,
        "m" => input.m,
        "a" => input.a,
        "s" => input.s,
    }
    .unwrap();

    loop {
        if cur == String::from("A") {
            return true;
        };
        if cur == String::from("R") {
            return false;
        };

        let rule = rules.get(&cur).unwrap();

        for decision in &rule.decisions {
            let evaluate = eval_with_context(&decision.expr, &context);

            if evaluate == Ok(Value::from(true)) {
                cur = decision.target.clone();
                break;
            }
        }
    }
}

fn main() {
    let (rules, inputs) = parse_input();

    let mut result = 0;
    for input in inputs {
        let accept = shall_accept(&input, &rules);
        if accept {
            result += input.x + input.m + input.a + input.s;
        }
    }
    println!("{result}");
}
