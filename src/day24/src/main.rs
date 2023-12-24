use regex::Regex;
use std::io;

const MIN: f64 = 200000000000000.0;
const MAX: f64 = 400000000000000.0;

#[derive(Debug)]
struct HailStone {
    x: i64,
    y: i64,
    z: i64,
    u: i64,
    v: i64,
    w: i64,
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().map(Result::unwrap).collect();
    let re =
        Regex::new(r"(-?\d+),\s*(-?\d+),\s*(-?\d+)\s*@\s*(-?\d+),\s+(-?\d+),\s(-?\d+)").unwrap();

    let mut hailstones = vec![];

    for line in lines {
        let (_, [x, y, z, u, v, w]) = re.captures(&line).unwrap().extract();
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        let z = z.parse().unwrap();
        let u = u.parse().unwrap();
        let v = v.parse().unwrap();
        let w = w.parse().unwrap();

        hailstones.push(HailStone { x, y, z, u, v, w });
    }

    println!("{hailstones:#?}");

    let mut res = 0;

    for (i1, h1) in hailstones.iter().enumerate() {
        for (i2, h2) in hailstones.iter().enumerate() {
            if i1 >= i2 {
                continue;
            }

            let nom = (h1.x - h2.x) * h2.v - (h1.y - h2.y) * h2.u;
            let den = h1.u * h2.v - h2.u * h1.v;

            if den == 0 {
                println!("skip");
                continue;
            }

            let a = -(nom as f64 / den as f64);

            if a < 0.0 {
                println!("previous A!");
                continue;
            }

            let x_now = (h1.x as f64) + a * (h1.u as f64);
            let y_now = (h1.y as f64) + a * (h1.v as f64);

            let b = (x_now - h2.x as f64) / h2.u as f64;

            if b < 0.0 {
                println!("previous B!");
                continue;
            }

            println!("{x_now} {y_now}");

            if x_now >= MIN && x_now <= MAX && y_now >= MIN && y_now <= MAX {
                println!("taken!");
                res += 1;
            }
        }
    }

    println!("{res}");
}
