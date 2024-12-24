use std::collections::{HashMap, HashSet};

fn main() {
    let data = include_str!("../input_1.txt");
    let hm = read(data);

    solve1(&hm);

    solve2(&hm);
}

fn read(data: &str) -> HashMap<String, HashSet<String>> {
    let mut hm = HashMap::<String, HashSet<String>>::new();

    for l in data.lines() {
        let (a, b) = l.split_once('-').unwrap();
        match hm.get_mut(a) {
            None => {
                let mut hs = HashSet::new();
                hs.insert(b.to_string());
                hm.insert(a.to_string(), hs);
            }
            Some(hs) => {
                hs.insert(b.to_string());
            }
        }

        match hm.get_mut(b) {
            None => {
                let mut hs = HashSet::new();
                hs.insert(a.to_string());
                hm.insert(b.to_string(), hs);
            }
            Some(hs) => {
                hs.insert(a.to_string());
            }
        }
    }

    hm
}

fn solve1(hm: &HashMap<String, HashSet<String>>) {
    let keys = hm.keys().cloned().collect::<Vec<_>>();

    let mut count = 0;
    for i in 0..keys.len() {
        for j in i + 1..keys.len() {
            for k in j + 1..keys.len() {
                let (a, b, c) = (&keys[i], &keys[j], &keys[k]);
                if !a.starts_with('t') && !b.starts_with('t') && !c.starts_with('t') {
                    continue;
                }

                if let Some(hs) = hm.get(a) {
                    if !hs.contains(b) || !hs.contains(c) {
                        continue;
                    }
                }

                if let Some(hs) = hm.get(b) {
                    if !hs.contains(a) || !hs.contains(c) {
                        continue;
                    }
                }

                if let Some(hs) = hm.get(c) {
                    if !hs.contains(b) || !hs.contains(a) {
                        continue;
                    }
                }

                count += 1;
            }
        }
    }

    println!("solve1: {}", count)
}

// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
/*
finds the maximal cliques that include all of the vertices in R, some of the vertices in P, and none of the vertices in X

algorithm BronKerbosch1(R, P, X) is
    if P and X are both empty then
        report R as a maximal clique
    for each vertex v in P do
        BronKerbosch1(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
        P := P \ {v}
        X := X ⋃ {v}
*/
fn bron_kerbosh1(
    edges: &HashMap<String, HashSet<String>>,
    r: HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
) -> HashSet<String> {
    if p.is_empty() && x.is_empty() {
        return r.clone();
    }

    let mut largest_clique = HashSet::new();

    for v in p.clone().iter() {
        let p_edges = match edges.get(v) {
            None => HashSet::new(),
            Some(e) => e.clone(),
        };

        let mut new_r = r.clone();
        new_r.insert(v.clone());
        let mut new_p = p.intersection(&p_edges).cloned().clone().collect();
        let mut new_x = x.intersection(&p_edges).cloned().clone().collect();
        let new_clique = bron_kerbosh1(edges, new_r, &mut new_p, &mut new_x);
        if new_clique.len() > largest_clique.len() {
            largest_clique = new_clique
        }

        p.remove(v);
        x.insert(v.clone());
    }

    largest_clique
}

// CRE:https://gist.github.com/icub3d/b98fd7a660535c41a664f7b2c12d0d81
fn solve2(edges: &HashMap<String, HashSet<String>>) {
    let clique = bron_kerbosh1(
        edges,
        HashSet::new(),
        &mut HashSet::from_iter(edges.keys().cloned()),
        &mut HashSet::new(),
    );

    let mut vertices = clique.iter().cloned().collect::<Vec<_>>();
    vertices.sort();

    println!("solve2: {}", vertices.join(","));
}
