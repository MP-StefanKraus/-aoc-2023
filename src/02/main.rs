use std::io;

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn main() {
    let mut sum = 0;
    for (game_num, line) in io::stdin().lines().enumerate() {
        let mut valid = true;

        let line = line.unwrap();

        let game_part = line.split(":").collect::<Vec<&str>>()[1];
        let draws = game_part.split(";");

        for draw in draws {
            let drawed_cubes = draw.split(",");
            for draw in drawed_cubes {
                let mut cubes = draw.trim().split(" ");

                let num: i32 = cubes.next().unwrap().parse().unwrap();
                let color = cubes.next().unwrap();

                match color {
                    "red" => valid &= num <= MAX_RED,
                    "blue" => valid &= num <= MAX_BLUE,
                    "green" => valid &= num <= MAX_GREEN,
                    _ => (),
                }
            }
        }

        if valid { sum += game_num + 1}
    }

    println!("{sum}");
}
