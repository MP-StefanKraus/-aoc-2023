use std::io;

fn simulate(numbers: Vec<i32>) -> i32 {

    let mut results = vec![];

    let mut cur = numbers;
    while cur.len() > 1 {
        results.push(cur[0]);

        let mut step = vec![];
        for i in 0..cur.len()-1 {
            let prev = cur[i];
            let nxt = cur[i+1];
            step.push(nxt-prev);
        }
        cur = step;
    }

    let mut res = 0;
    for num in results.iter().rev() {
        res = num - res;
    }

    res
}

fn main() {

    let mut sum = 0;
    for line in io::stdin().lines() {
        let numbers: Vec<_> = line.unwrap().split(' ').map(|x| x.parse::<i32>().unwrap()).collect();

        let res = simulate(numbers);

        sum += res;
    }

    println!("{sum}");
}



