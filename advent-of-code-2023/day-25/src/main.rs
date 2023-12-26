// new thing to learn everyday: Minimum cut & Karger's algorithm

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    io::Read,
};

use rand::Rng;

fn main() {
    const INPUT: &str = "input.txt";
    let g = p1_parse_input(INPUT);
    let removed = p1_karger_min_cut(&g);
    p1_count_disjoint(&g, &removed);
}

fn p1_count_disjoint(graph: &Graph, removed_edges: &[(String, String)]) {
    let mut adj_list: HashMap<String, HashSet<String>> = HashMap::new();
    for v in graph.vertices.iter() {
        adj_list.insert(v.clone(), HashSet::new());
    }
    for e in graph.edges.iter() {
        if removed_edges.contains(e) {
            continue;
        }
        match adj_list.get_mut(&e.0) {
            None => {
                adj_list.insert(e.0.clone(), HashSet::from_iter([e.1.clone()]));
            }
            Some(s) => {
                s.insert(e.1.clone());
            }
        }
        match adj_list.get_mut(&e.1) {
            None => {
                adj_list.insert(e.1.clone(), HashSet::from_iter([e.0.clone()]));
            }
            Some(s) => {
                s.insert(e.0.clone());
            }
        }
    }

    let mut disjoint_sets = HashMap::<String, String>::new(); // node -> label
    for src in adj_list.keys() {
        let mut label: Option<String> = None;
        let mut queue = VecDeque::<String>::new();
        let mut visited = HashSet::<String>::new();
        let mut visiting = HashSet::<String>::new();
        queue.push_front(src.clone());
        while let Some(node) = queue.pop_back() {
            // println!("visiting {}", node);
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node.clone());
            visiting.remove(&node);

            for d in adj_list.get(&node).unwrap().iter() {
                if visited.contains(d) || visiting.contains(d) {
                    continue;
                }
                // println!("pushed {} {}", src.clone(), d.clone());
                queue.push_front(d.clone());
                visiting.insert(d.clone());
            }

            if label.is_some() {
                continue;
            }

            if let Some(lb) = disjoint_sets.get(&node) {
                label = Some(lb.clone())
            }
        }
        if label.is_none() {
            label = Some(src.clone())
        }
        let label = label.unwrap();
        for n in visited {
            disjoint_sets.insert(n, label.clone());
        }
    }

    let mut count = HashMap::<String, usize>::new();
    for (_, label) in disjoint_sets.iter() {
        match count.get_mut(label) {
            None => {
                count.insert(label.clone(), 1);
            }
            Some(v) => {
                *v += 1;
            }
        }
    }

    println!("{:?}", count);
}

fn p1_karger_min_cut(graph: &Graph) -> Vec<(String, String)> {
    let mut rng = rand::thread_rng();

    loop {
        let mut g = graph.clone();
        let mut parents = HashMap::<String, String>::new();
        let mut rank = HashMap::<String, usize>::new();
        for v in g.vertices.iter() {
            parents.insert(v.clone(), v.clone());
            rank.insert(v.clone(), 0);
        }

        fn parents_of(parents: &mut HashMap<String, String>, node: &str) -> String {
            let p = parents.get(node).unwrap().clone();
            if p.eq(node) {
                return p.clone();
            }
            parents_of(parents, &p)
        }

        let mut count = g.vertices.len();
        let mut edges: Vec<(String, String)> = g.edges.iter().cloned().collect();
        loop {
            let idx = rng.gen_range(0..edges.len());
            let edge = edges[idx].clone();
            if count <= 2 {
                break;
            }
            let (pu, pv) = (
                parents_of(&mut parents, &edge.0),
                parents_of(&mut parents, &edge.1),
            );
            if pu != pv {
                // union
                let (pu, pv) = (parents_of(&mut parents, &pu), parents_of(&mut parents, &pv));
                let (ru, rv) = (*rank.get(&pu).unwrap(), *rank.get(&pv).unwrap());
                if ru > rv {
                    *parents.get_mut(&pv).unwrap() = pu.clone();
                }
                if ru < rv {
                    *parents.get_mut(&pu).unwrap() = pv.clone();
                }
                if ru == rv {
                    *parents.get_mut(&pv).unwrap() = pu.clone();
                    *rank.get_mut(&pu).unwrap() += 1;
                }
                count -= 1;
            }

            g.edges.remove(&edge);
            edges.remove(idx);
            // println!("removed {:?}", edge)
        }

        let mut min_cut = 0;
        let mut cut_edges = Vec::<(String, String)>::new();
        for e in g.edges {
            if parents_of(&mut parents, &e.0) != parents_of(&mut parents, &e.1) {
                min_cut += 1;
                cut_edges.push(e.clone())
            }
        }
        if min_cut == 3 {
            println!("{:?}", cut_edges);
            // println!("{:?}", rank);
            return cut_edges;
        }
    }
}

fn p1_parse_input(file: &str) -> Graph {
    let mut buffer = String::new();
    fs::File::open(file)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let mut adj_list: HashMap<String, HashSet<String>> = HashMap::new();

    for l in buffer.lines().filter(|l| !l.is_empty()) {
        let (src, dests) = l.split_once(": ").unwrap();
        let dests: Vec<String> = dests
            .trim()
            .split(' ')
            .filter(|v| !v.is_empty())
            .map(|s| s.to_string())
            .collect();

        match adj_list.get_mut(src) {
            None => {
                adj_list.insert(src.to_string(), HashSet::from_iter(dests.iter().cloned()));
            }
            Some(adjs) => {
                for d in dests {
                    adjs.insert(d);
                }
            }
        };
    }

    let mut vertices: HashSet<String> = HashSet::new();
    let mut edges: HashSet<(String, String)> = HashSet::new();
    for (src, dests) in adj_list.iter() {
        vertices.insert(src.clone());
        for dst in dests.iter() {
            vertices.insert(dst.clone());
            if src > dst {
                edges.insert((src.clone(), dst.clone()));
            } else {
                edges.insert((dst.clone(), src.clone()));
            }
        }
    }

    Graph { vertices, edges }
}

#[derive(Clone)]
struct Graph {
    vertices: HashSet<String>,
    edges: HashSet<(String, String)>,
}
