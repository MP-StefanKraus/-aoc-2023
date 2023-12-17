use std::io;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct Position (isize, isize);
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct Direction (isize, isize);

fn get_left_dir(dir: &Direction) -> Direction {
    match dir {
        &UP => LEFT,
        &LEFT => DOWN,
        &DOWN => RIGHT,
        &RIGHT => UP,
        _ => panic!(),
    }
}

fn get_right_dir(dir: &Direction) -> Direction {
    get_left_dir(&get_left_dir(&get_left_dir(dir)))
}

const UP: Direction = Direction(-1, 0);
const DOWN: Direction = Direction(1, 0);
const LEFT: Direction = Direction(0, -1);
const RIGHT: Direction = Direction(0, 1);

#[derive(PartialEq, Eq)]
struct State {
    pos: Position,
    dir: Direction,
    num_of_same: usize,
    sum: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.sum.cmp(&self.sum)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let field: Vec<Vec<usize>> = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect();


    let mut heap = BinaryHeap::new();

    heap.push(State {pos: Position(0, 0), dir: RIGHT, num_of_same: 0, sum: 0});
    heap.push(State{pos: Position(0, 0), dir: DOWN, num_of_same: 0, sum: 0});

    let endpos = Position(field.len() as isize - 1, field[0].len() as isize -1);
    let result;

    let mut visited: HashMap<(Position, Direction, usize), usize> = HashMap::new();

    loop {
        let cur = heap.pop().unwrap();

        if cur.pos.0 < 0 || cur.pos.1 < 0 || cur.pos.0 >= field.len() as isize || cur.pos.1 >= field[0].len() as isize {
            continue;
        }

        let new_sum = cur.sum + field[cur.pos.0 as usize][cur.pos.1 as usize];

        // check and memoize smallest cost we found with same precondition
        let minimal = visited.get(&(cur.pos.clone(), cur.dir.clone(), cur.num_of_same)).unwrap_or(&10000000000);
        if *minimal <= new_sum {
            continue;
        }
        //println!("new or better: {:?},{:?},{:?} {new_sum} < {minimal}  ", cur.pos.clone(), cur.dir.clone(), cur.num_of_same);
        visited.insert((cur.pos.clone(), cur.dir.clone(), cur.num_of_same), new_sum);


        if cur.pos == endpos {
            result = new_sum;
            break;
        }

        let mut possible_dirs_and_same = Vec::new();
        possible_dirs_and_same.push((get_left_dir(&cur.dir), 1));
        possible_dirs_and_same.push((get_right_dir(&cur.dir), 1));
        if cur.num_of_same < 3 {
            possible_dirs_and_same.push((cur.dir.clone(), cur.num_of_same+1))
        }

        for (new_dir, same) in possible_dirs_and_same {
            let new_pos = Position(cur.pos.0 + new_dir.0, cur.pos.1 + new_dir.1);

            heap.push(State {pos: new_pos, dir: new_dir, num_of_same: same, sum: new_sum } )
        }
    }

    let fixed_result = result - field[0][0];

    println!("{fixed_result}");
}
