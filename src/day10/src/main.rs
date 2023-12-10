use std::io;

const DIRECTIONS: [((isize, isize), [char; 4], [char; 4]); 4] = [
    ((-1, 0), ['S', '|', 'L', 'J'], ['S', '|', 'F', '7']),
    ((1, 0), ['S', '|', 'F', '7'], ['S', '|', 'L', 'J']),
    ((0, -1), ['S', '-', 'J', '7'], ['S', '-', 'F', 'L']),
    ((0, 1), ['S', '-', 'F', 'L'], ['S', '-', 'J', '7']),
];

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(|x| x.unwrap()).collect();

    let mut visited: Vec<Vec<bool>> = lines
        .iter()
        .map(|x| x.chars().map(|_f| false).collect())
        .collect();

    let mut q = vec![];

    for (i, line) in lines.iter().enumerate() {
        match line.find('S') {
            Some(y) => q.push((i, y)),
            None => (),
        }
    }

    assert_eq!(q.len(), 1);

    while !q.is_empty() {
        let nxt = q.pop().unwrap();
        if visited[nxt.0][nxt.1] {
            continue;
        }
        visited[nxt.0][nxt.1] = true;

        for (dir, cur, next_assume) in DIRECTIONS.iter() {
            let nx = (nxt.0 as isize) + dir.0;
            let ny = (nxt.1 as isize) + dir.1;

            if !(0 <= nx && nx < visited.len() as isize && 0 <= ny && ny < visited[0].len() as isize) {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            if cur.contains(&lines[nxt.0].chars().nth(nxt.1).unwrap())
                && next_assume.contains(&lines[nx].chars().nth(ny).unwrap())
            {
                q.push((nx, ny));
            }
        }
    }

    let doubled_res: usize = visited.iter().map(|y| y.iter().filter(|x| **x).count()).sum();
    let res = doubled_res / 2;
    println!("{res:#?}")
}
