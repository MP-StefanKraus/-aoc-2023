use std::cmp;
use std::io;

fn main() {
    let mut sum = 0;
    for (_game_num, line) in io::stdin().lines().enumerate() {
        let line = line.unwrap();

        let game_part = line.split(":").collect::<Vec<&str>>()[1];
        let draws = game_part.split(";");

        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        for draw in draws {
            let drawed_cubes = draw.split(",");
            for draw in drawed_cubes {
                let mut cubes = draw.trim().split(" ");

                let num: i32 = cubes.next().unwrap().parse().unwrap();
                let color = cubes.next().unwrap();

                match color {
                    "red" => max_red = cmp::max(max_red, num),
                    "blue" => max_blue = cmp::max(max_blue, num),
                    "green" => max_green = cmp::max(max_green, num),
                    _ => (),
                }
            }
        }

        let power = max_red * max_blue * max_green;
        sum += power;
    }

    println!("{sum}");
}
