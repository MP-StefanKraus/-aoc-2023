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

    let mut numbers = vec![];

    for c in 0..1020 {
        let mut new_positions = HashSet::new();

        for p in positions {
            for d in DIRS {
                let nx = (p.0 + d.0);
                let ny = (p.1 + d.1);

                let mx = field.len() as isize;
                let my = field[0].len() as isize;

                let access_pos_x = (((nx % mx) + mx) % mx) as usize;
                let access_pos_y = (((ny % my) + my) % my) as usize;

                let char_at = field[access_pos_x][access_pos_y];
                if char_at == '.' || char_at == 'S' {
                    new_positions.insert((nx, ny));
                }
            }
        }

        numbers.push((c + 1, new_positions.len()));

        positions = new_positions;
    }

    println!("{numbers:#?}");

    // from here on, I took python for some linear regression on F(65), F(65+131), F(65+131*2).
    println!("{}", 621494544278648);
}
