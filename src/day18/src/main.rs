use regex::Regex;
use std::io;

#[derive(Debug)]
struct Instruction {
    dir: i8,
    steps: i64,
}

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(|x| x.unwrap()).collect();

    let mut instructions: Vec<Instruction> = vec![];

    let re = Regex::new(r"\(#([\w\d]+)([0-3])\)").unwrap();
    for l in lines {
        for (_, [steps, dir]) in re.captures_iter(l.as_str()).map(|c| c.extract()) {
            let dir = dir.to_string().parse().unwrap();
            let steps = u32::from_str_radix(steps, 16).unwrap() as i64;

            instructions.push(Instruction { dir, steps });
        }
    }

    let mut cur_point = (0, 0);

    let mut points = vec![];

    let mut points_on_coords = 0;
    for instr in instructions.iter() {
        match instr.dir {
            0 => cur_point = (cur_point.0, cur_point.1 + instr.steps),
            2 => cur_point = (cur_point.0, cur_point.1 - instr.steps),
            3 => cur_point = (cur_point.0 - instr.steps, cur_point.1),
            1 => cur_point = (cur_point.0 + instr.steps, cur_point.1),
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
