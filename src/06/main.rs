use std::io;

fn main() {

    let mut time = String::new();
    let mut distance = String::new();

    let _ = io::stdin().read_line(&mut time);
    _ = io::stdin().read_line(&mut distance);

    let mut timestrings = time.split_whitespace().collect::<Vec<_>>();
    let mut distancestrings = distance.split_whitespace().collect::<Vec<_>>();
    timestrings.remove(0);
    distancestrings.remove(0);

    let times: Vec<_> = timestrings.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    let distances: Vec<_> = distancestrings.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut result = 1;
    for i in 0..times.len() {
        let t = times[i];
        let d = distances[i];

        let mut num = 0;
        for j in 0..t+1 {
            if (t - j) * j > d {
                num += 1;
            }
        }
        println!("{num}");

        result *= num;
    }

    println!("{result}")

}
