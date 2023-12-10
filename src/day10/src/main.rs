use std::io;

const DIRECTIONS: [((isize, isize), [char; 4], [char; 4]); 4] = [
    ((-1, 0), ['S', '|', 'L', 'J'], ['S', '|', 'F', '7']),
    ((1, 0), ['S', '|', 'F', '7'], ['S', '|', 'L', 'J']),
    ((0, -1), ['S', '-', 'J', '7'], ['S', '-', 'F', 'L']),
    ((0, 1), ['S', '-', 'F', 'L'], ['S', '-', 'J', '7']),
];

fn main() {
    let lines: Vec<_> = io::stdin().lines().map(|x| x.unwrap()).collect();

    let mut visited: Vec<Vec<i32>> = lines
        .iter()
        .map(|x| x.chars().map(|_f| 0).collect())
        .collect();

    let mut q = vec![];

    for (i, line) in lines.iter().enumerate() {
        match line.find('S') {
            Some(y) => q.push((i, y)),
            None => (),
        }
    }

    assert_eq!(q.len(), 1);

    let mut cur = 0;
    while !q.is_empty() {
        let nxt = q.pop().unwrap();
        if visited[nxt.0][nxt.1] > 0 {
            continue;
        }
        cur += 1;
        visited[nxt.0][nxt.1] = cur;

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

    let points_on_coords: usize = visited.iter().map(|y| y.iter().filter(|x| **x > 0).count()).sum();

    let mut corners = vec![];

    for (i, line) in lines.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if visited[i][j] > 0 && "SFJ7L".contains(char) {
                corners.push((visited[i][j], (i as isize, j as isize)));
            }
        }
    }

    corners.sort();

    let mut doubled_area = 0;

    for i in 0..corners.len() {
        let c = corners[i].1;
        let n = corners[(i+1) % corners.len()].1;

        doubled_area += (c.1 + n.1) * (c.0 - n.0);
    }

    if doubled_area < 0 {
        doubled_area *= -1;
    }

    let area = doubled_area / 2;

    let R = (area + 1 - (points_on_coords as isize) / 2);

    println!("{R:#?}")
}
