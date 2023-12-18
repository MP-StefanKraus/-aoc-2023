use regex::Regex;
use std::io;

#[derive(Debug)]
struct Instruction {
    dir: char,
    steps: i32,
    color: String,
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(|x| x.unwrap()).collect();

    let mut instructions: Vec<Instruction> = vec![];

    let re = Regex::new(r"(\w) (\d+) \(#([\w\d]+)\)").unwrap();
    for l in lines {
        for (_, [dir, steps, color]) in re.captures_iter(l.as_str()).map(|c| c.extract()) {
            let dir = dir.chars().next().unwrap();
            let steps = steps.parse().unwrap();
            let color = color.to_string();

            instructions.push(Instruction { dir, steps, color });
        }
    }

    let mut cur_point = (0, 0);

    let mut points = vec![];

    let mut points_on_coords = 0;
    for instr in instructions.iter() {
        match instr.dir {
            'R' => cur_point = (cur_point.0, cur_point.1 + instr.steps),
            'L' => cur_point = (cur_point.0, cur_point.1 - instr.steps),
            'U' => cur_point = (cur_point.0 - instr.steps, cur_point.1),
            'D' => cur_point = (cur_point.0 + instr.steps, cur_point.1),
            _ => panic!(),
        };

        points.push(cur_point);
        points_on_coords += instr.steps;
    }

    let mut doubled_area = 0;

    for i in 0..points.len() {
        let c = points[i];
        let n = points[(i + 1) % points.len()];

        doubled_area += (c.1 + n.1) * (c.0 - n.0);
    }

    if doubled_area < 0 {
        doubled_area *= -1;
    }

    let area = doubled_area / 2;
    let inner_points = area + 1 - (points_on_coords) / 2;
    let result = points_on_coords + inner_points;

    println!("{result}");
}
