use regex::Regex;
use std::arch::x86_64::_mm_minpos_epu16;
use std::io;

const MIN: i64 = -500;
const MAX: i64 = 500;

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

    let p1 = &hailstones[0];
    let p2 = &hailstones[1];

    let mut potential_results = vec![];

    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    let dz = p1.z - p2.z;

    // Find possible solutions where we have integer solutions through
    // two hailstone lines, finds too much but prunes search space well.
    // math was done on paper, and assumption about velocity was done.

    for u in MIN..MAX + 1 {
        for v in MIN..MAX + 1 {
            for w in MIN..MAX + 1 {
                let du1 = u - p1.u;
                let du2 = u - p2.u;
                let dv1 = v - p1.v;
                let dv2 = v - p2.v;
                let dw1 = w - p1.w;
                let dw2 = w - p2.w;

                let num1 = dx * dv2 - dy * du2;
                let den1 = du1 * dv2 - dv1 * du2;

                if den1 == 0 {
                    continue;
                }

                if num1 % den1 != 0 {
                    continue;
                }

                let l1 = num1 / den1;

                if l1 < 0 {
                    continue;
                }

                let num2 = l1 * dv1 - dy;
                let den2 = dv2;

                if den2 == 0 {
                    continue;
                }

                if num2 % den2 != 0 {
                    continue;
                }

                let l2 = num2 / den2;

                if l2 < 0 {
                    continue;
                }

                if l1 * dw1 - l2 * dw2 != dz {
                    continue;
                }

                let x1 = p1.x - l1 * du1;
                let y1 = p1.y - l1 * dv1;
                let z1 = p1.z - l1 * dw1;

                potential_results.push((x1, y1, z1, u, v, w));
            }
        }
    }

    // now validate which ones match all hailstone collisions.
    for (x, y, z, u, v, w) in potential_results {
        let mut possible = true;

        for hailstone in hailstones.iter() {
            let dx = hailstone.x - x;
            let du = hailstone.u - u;
            if dx % du != 0 {
                possible = false;
                break;
            }
            let l = dx / du;

            possible &= hailstone.x + l * hailstone.u != x + l * u
                && hailstone.y + l * hailstone.u != y + l * v
                && hailstone.z + l * hailstone.u != z + l * w
        }

        if possible {
            let res = x + y + z;
            println!("{x}, {y}, {z} => {res}");
            break;
        }
    }
}
