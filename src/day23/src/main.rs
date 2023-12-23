use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::io;

const DIR: [((isize, isize), char); 4] =
    [((-1, 0), 'v'), ((1, 0), '^'), ((0, -1), '>'), ((0, 1), '<')];


fn detect_vertice_positions(field: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut result = vec![];

    for i in 1..field.len() - 1 {
        let line = &field[i];
        for j in 1..field[i].len() - 1 {
            let ch = line[j];
            if ch == '#' {
                continue;
            }

            let mut only_slopes = true;

            for (d, _) in DIR.iter() {
                let i_pos = (i as isize + d.0, j as isize + d.1);
                let (nx, ny) = (i_pos.0 as usize, i_pos.1 as usize);

                let nc = field[nx][ny];
                if nc == '.' {
                    only_slopes = false;
                    break;
                }
            }

            if only_slopes {
                result.push((i, j));
            };
        }
    }

    result.push((0, 1));
    result.push((field.len() - 1, field[0].len() - 2));

    result
}

fn find_neighbour_nodes(field: &Vec<Vec<char>>, nodes: &Vec<(usize, usize)>, cur: &(usize, usize), prev: &(usize, usize), way: usize) -> Vec<((usize, usize), usize)> {

    let mut result = vec![];

    if nodes.contains(cur) && cur != prev {
        result.push((*cur, way));
        return result;
    };

    for (d, _) in DIR.iter() {
        let i_pos = (cur.0 as isize + d.0, cur.1 as isize + d.1);
        let new_pos = (i_pos.0 as usize, i_pos.1 as usize);

        if new_pos == *prev {
            continue;
        }
        if new_pos.0 >= field.len() || new_pos.1 >= field[0].len() {
            continue;
        }
        let waychar = field[new_pos.0][new_pos.1];
        if waychar == '#' {
            continue;
        }

        let part_res = find_neighbour_nodes(field, nodes, &new_pos, cur, way+1);
        result.extend(part_res);
    }

    result
}

fn find_edges(field: &Vec<Vec<char>>, nodes: &Vec<(usize, usize)>) -> Vec<((usize, usize), (usize, usize), usize)> {
    let mut result = vec![];

    for node in nodes {
        let node_neighbours = find_neighbour_nodes(field, nodes, node, node, 0);
        for (neigh, way) in node_neighbours {
            result.push((*node, neigh, way));
        }
    }

    result
}

fn get_max_length(graph: &HashMap<(usize, usize), Vec<((usize, usize), usize)>>,
                  pos: (usize, usize),
                  visited: &mut HashSet<(usize, usize)>,
) -> isize {

    if pos.0 == 0 {
        return 0;
    }

    let mut mx = -1000;

    let neighbours = graph.get(&pos).unwrap();
    for (np, cost) in neighbours.iter() {

        if visited.contains(np) {
            continue;
        }

        visited.insert(*np);
        mx = max(mx, get_max_length(graph, *np, visited) + (*cost as isize));

        visited.remove(np);
    }

    return mx;
}

fn main() {
    let field: Vec<Vec<char>> = io::stdin()
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    let vertice_positions = detect_vertice_positions(&field);
    let edges = find_edges(&field, &vertice_positions);

    let mut graph: HashMap<(usize, usize), Vec<((usize, usize), usize)>>    = HashMap::new();
    for pos in vertice_positions {
        graph.insert(pos, vec![]);
    }
    for edge in edges {
        let (from, to, cost) = edge;

        let neighbours = graph.get_mut(&from).unwrap();
        neighbours.push((to, cost));
    }

    let end = (field.len()-1, field[0].len()-2);
    let mut visited = HashSet::new();

    let result = get_max_length(&graph, end, &mut visited);

    println!("{result}");
}
