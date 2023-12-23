use std::cmp::max;
use std::io;

const DIR: [((isize, isize), char); 4] = [
    ((-1, 0), 'v'),
    ((1, 0), '^'),
    ((0, -1), '>'),
    ((0, 1), '<'),
];

fn get_max_length(field: &Vec<Vec<char>>, pos: (isize, isize), last_pos: (isize, isize)) -> isize {

    if pos.0 == 0 {
        return 0
    }

    let mut mx = 0;

    for (d, a) in DIR.iter() {

        let i_pos = (pos.0 + d.0, pos.1 + d.1);
        let new_pos = (i_pos.0 as usize, i_pos.1 as usize);

        if i_pos == last_pos {
            continue;
        }
        if new_pos.0 >= field.len() || new_pos.1 >= field[0].len() {
            continue;
        }

        let waychar =  field[new_pos.0][new_pos.1];

        if waychar != '.' && waychar != *a {
            continue;
        }

        mx = max(mx, get_max_length(field, i_pos, pos));
    };

    return 1 + mx;
}

fn main() {
    println!("Hello, world!");
    let field: Vec<Vec<char>> = io::stdin().lines().map(|x| x.unwrap().chars().collect()).collect();

    let cur_pos = ((field.len()-1) as isize, (field[0].len()-2) as isize);

    let result = get_max_length(&field, cur_pos, (0, 0));

    println!("{result}");
}
