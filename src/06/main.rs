use std::io;

fn main() {

    let mut time = String::new();
    let mut distance = String::new();

    let _ = io::stdin().read_line(&mut time);
    _ = io::stdin().read_line(&mut distance);

    let time_str = time.split_once(" ").unwrap().1.replace(" ", "");
    let time = time_str.trim().parse::<u64>().unwrap();
    let distance_str = distance.split_once(" ").unwrap().1.replace(" ", "");
    let distance = distance_str.trim().parse::<u64>().unwrap();

    let mut result = 0;

    for j in 0..time+1 {
        if (time - j) * j > distance {
            result += 1;
        }
    }

    println!("{result}")

}
