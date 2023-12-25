use std::collections::{HashMap, HashSet};
use std::io;
use pathfinding;

fn main() {
    let mut nodes: HashSet<String> = HashSet::new();
    let mut connections: Vec<(String, String)> = vec![];

    for line in io::stdin().lines() {
        let line = line.unwrap();
        let (from, tos) = line.split_once(":").unwrap();
        nodes.insert(from.to_string());
        for to in tos.trim().split(" ") {
            connections.push((from.to_string(), to.to_string()));
            nodes.insert(to.to_string());
        }
    }

    let mut node_intmap = HashMap::new();
    for (i, n) in nodes.iter().enumerate() {
        node_intmap.insert(n.clone(), i);
    }

    let v: Vec<usize> = nodes.into_iter().map(|x| *(node_intmap.get(&x).unwrap())).collect();
    let e = connections.into_iter().fold(vec![], |mut acc, (a, b)| {
        let a_i = *node_intmap.get(&a).unwrap();
        let b_i = *node_intmap.get(&b).unwrap();
        acc.push(((a_i, b_i), 1));
        acc.push(((b_i, a_i), 1));

        acc
    });


    'outer: for i in 0..v.len() {
        for j in i+1..v.len() {
            let res = pathfinding::directed::edmonds_karp::edmonds_karp_sparse(&v, &v[i], &v[j], e.clone());
            if res.1 == 3 {

                let cut_edges: Vec<(usize, usize)> = res.2.iter().map(|x| (x.0.0, x.0.1)).collect();

                let without_cut: Vec<(usize, usize)> = e.clone().iter().map(|x| x.0).filter(|y| !(cut_edges.contains(y) || cut_edges.contains(&(y.1, y.0)))).collect();
                let mut neighbour_map: HashMap<usize, Vec<usize>> = HashMap::new();
                for (a, b) in without_cut.into_iter() {
                    neighbour_map.entry(a).or_insert(vec![]).push(b);
                    neighbour_map.entry(b).or_insert(vec![]).push(a);
                }

                let neighbour_func = |&n: &_| neighbour_map.get(&n).unwrap().clone();

                let r = pathfinding::directed::strongly_connected_components::strongly_connected_components(
                    &v, neighbour_func
                );

                assert_eq!(r.len(), 2);
                let l1 = r[0].len();
                let l2 = r[1].len();

                println!("{}, {} -> {}", l1, l2, l1*l2);

                break 'outer;
            }
        }
    }
}
