pub struct Solution {}

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut edges = Vec::<Edge>::with_capacity(points.len());
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                edges.push(Edge {
                    from: i,
                    to: j,
                    dist: manhattan_distance(&points[i], &points[j]),
                })
            }
        }
        edges.sort_by(|a, b| a.dist.cmp(&b.dist));

        let mut parents: Vec<usize> = (0..points.len()).collect();
        let mut ranks: Vec<usize> = vec![0; points.len()];
        let mut dist = 0;
        for e in edges {
            let (p_from, p_to) = (
                Self::find(&mut parents, e.from),
                Self::find(&mut parents, e.to),
            );
            if p_from == p_to {
                continue;
            }
            Self::union(&mut parents, &mut ranks, p_from, p_to);

            dist += e.dist;
        }

        dist
    }

    pub fn find(parents: &mut [usize], a: usize) -> usize {
        if parents[a] == a {
            return a;
        }

        let p = Self::find(parents, parents[a]);
        parents[a] = p;

        p
    }

    pub fn union(parents: &mut [usize], ranks: &mut [usize], a: usize, b: usize) {
        let (par_a, par_b) = (Self::find(parents, a), Self::find(parents, b));
        if par_a == par_b {
            return;
        }
        if ranks[par_a] > ranks[par_b] {
            parents[par_b] = par_a;
            return;
        }
        if ranks[par_b] > ranks[par_a] {
            parents[par_a] = par_b;
            return;
        }

        parents[par_a] = par_b;
        ranks[par_b] += 1;
    }
}

fn manhattan_distance(a: &[i32], b: &[i32]) -> i32 {
    i32::abs(a[0] - b[0]) + i32::abs(a[1] - b[1])
}

pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub dist: i32,
}
