use std::collections::HashSet;
use std::io;

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let field: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    let mut positions = HashSet::new();
    for (i, chars) in field.iter().enumerate() {
        let start = chars.iter().position(|x| *x == 'S');
        match start {
            None => {}
            Some(j) => {
                positions.insert((i as isize, j as isize));
            }
        };
    }

    for _ in 0..64 {
        let mut new_positions = HashSet::new();

        for p in positions {
            for d in DIRS {
                let nx = p.0 + d.0;
                let ny = p.1 + d.1;

                if !(nx >= 0
                    && nx < field.len() as isize
                    && ny >= 0
                    && ny < field[0].len() as isize)
                {
                    continue;
                }

                let char_at = field[nx as usize][ny as usize];
                if char_at == '.' || char_at == 'S' {
                    new_positions.insert((nx, ny));
                }
            }
        }

        positions = new_positions;
    }

    println!("{}", positions.iter().count());
}
