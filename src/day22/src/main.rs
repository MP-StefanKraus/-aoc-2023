use rayon::prelude::*;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::io;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct Brick {
    start_cube: Cube,
    end_cube: Cube,
    identifier: i32,
}

impl Cube {
    fn from_input_str(s: &str) -> Cube {
        let split: Vec<i32> = s.split(',').map(|x| x.parse().unwrap()).collect();
        let x = split[0];
        let y = split[1];
        let z = split[2];

        let cube = Cube { x, y, z };
        cube
    }
}

impl Brick {
    fn lower_once(self) -> Brick {
        if self.start_cube.z == 1 {
            return self;
        }
        Brick {
            start_cube: Cube {
                z: self.start_cube.z - 1,
                ..self.start_cube
            },
            end_cube: Cube {
                z: self.end_cube.z - 1,
                ..self.end_cube
            },
            identifier: self.identifier,
        }
    }

    fn generate_cubes(&self) -> HashSet<Cube> {
        let mut set = HashSet::new();
        for x in self.start_cube.x..self.end_cube.x + 1 {
            set.insert(Cube {
                x,
                y: self.start_cube.y,
                z: self.start_cube.z,
            });
        }
        for y in self.start_cube.y..self.end_cube.y + 1 {
            set.insert(Cube {
                x: self.start_cube.x,
                y,
                z: self.start_cube.z,
            });
        }
        for z in self.start_cube.z..self.end_cube.z + 1 {
            set.insert(Cube {
                x: self.start_cube.x,
                y: self.start_cube.y,
                z,
            });
        }

        set
    }
}

fn parse_input() -> Vec<Brick> {
    let mut bricks = vec![];

    for (i, line) in io::stdin().lines().enumerate() {
        let tmp = line.unwrap();
        let (a, b) = tmp.split_once('~').unwrap();

        let ca = Cube::from_input_str(a);
        let cb = Cube::from_input_str(b);

        let start_cube = min(ca, cb);
        let end_cube = max(ca, cb);

        let brick = Brick {
            start_cube,
            end_cube,
            identifier: i as i32,
        };
        bricks.push(brick);
    }

    bricks
}

fn single_downfall_step(bricks: &Vec<Brick>) -> Vec<Brick> {
    let mut current_cubes: HashSet<Cube> = HashSet::new();
    for brick in bricks.iter() {
        current_cubes.extend(brick.generate_cubes().iter())
    }

    let mut new_brickstate = vec![];

    for brick in bricks.iter() {
        let lowered_brick = brick.lower_once();
        let brick_cubes = brick.generate_cubes();
        let lowered_cubes = lowered_brick.generate_cubes();

        let removed_own_blocks: HashSet<Cube> =
            current_cubes.difference(&brick_cubes).map(|x| *x).collect();
        let blocking_blocks: HashSet<Cube> = removed_own_blocks
            .intersection(&lowered_cubes)
            .map(|x| *x)
            .collect();

        if blocking_blocks.is_empty() {
            new_brickstate.push(lowered_brick);
        } else {
            new_brickstate.push(*brick);
        }
    }

    new_brickstate
}

fn same_as_before(old: &Vec<Brick>, new: &Vec<Brick>) -> bool {
    return old == new;
}

fn downfall_until_fixed(bricks: &Vec<Brick>) -> Vec<Brick> {
    let mut previous = bricks.clone();
    loop {
        println!("Loopin...");
        let new_downfalled = single_downfall_step(&previous);

        //println!("{previous:#?}");
        //println!("{new_downfalled:#?}");

        if same_as_before(&previous, &new_downfalled) {
            return new_downfalled;
        }

        previous = new_downfalled;
    }
}

fn main() {
    let bricks = parse_input();

    let stabelized = downfall_until_fixed(&bricks);

    let fallen_brick_numbers: Vec<usize> = stabelized
        .par_iter()
        .enumerate()
        .map(|(i, _)| {
            let mut current_bricks = stabelized.clone();
            current_bricks.remove(i);

            let new_bricks = downfall_until_fixed(&current_bricks);
            let new_bricks_set: HashSet<Brick> = HashSet::from_iter(new_bricks);
            let previous_sets: HashSet<Brick> = HashSet::from_iter(current_bricks);

            let num_fallen = previous_sets.difference(&new_bricks_set).count();
            println!("{i}: {num_fallen}");
            num_fallen
        })
        .collect();

    let result: usize = fallen_brick_numbers.iter().sum();

    println!("{result}");
}
