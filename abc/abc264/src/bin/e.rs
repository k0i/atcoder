use proconio::{fastout, input};

#[fastout]
pub fn main() {
    input! {
        n: usize,
        m: usize,
        e: usize,
        uv: [(usize, usize); e],
        q: usize,
        xs: [usize; q],
    }

    let mut results: Vec<usize> = vec![];

    let mut sorted_xs = xs.clone();
    sorted_xs.sort();

    let mut parents: Vec<(usize, usize)> = vec![(0, 1); n + m];

    for i in 0..n {
        parents[i].0 = 1;
        parents[i].1 = 0;
    }

    let mut e_city: usize = 0;

    let mut uf = UnionFind::new(n + m);

    let mut j = 0;

    for (i, &(u, v)) in uv.iter().enumerate() {
        if j < q && i + 1 == sorted_xs[j] {
            j += 1;
            continue;
        }

        let up = uf.find(u - 1);
        let vp = uf.find(v - 1);

        if up == vp {
            continue;
        }

        uf.unite(u - 1, v - 1);
        let new_p = uf.find(u - 1);

        if parents[up].1 == 0 && parents[vp].1 > 0 {
            e_city += parents[up].0;
        }
        if parents[up].1 > 0 && parents[vp].1 == 0 {
            e_city += parents[vp].0;
        }

        parents[new_p].0 = parents[up].0 + parents[vp].0;
        parents[new_p].1 = parents[up].1 + parents[vp].1;
    }

    results.push(e_city);

    for &x in xs.iter().rev() {
        let (u, v) = uv[x - 1];

        let up = uf.find(u - 1);
        let vp = uf.find(v - 1);

        if up == vp {
            results.push(e_city);
            continue;
        }

        uf.unite(u - 1, v - 1);
        let new_p = uf.find(u - 1);

        if parents[up].1 == 0 && parents[vp].1 > 0 {
            e_city += parents[up].0;
        }
        if parents[up].1 > 0 && parents[vp].1 == 0 {
            e_city += parents[vp].0;
        }

        parents[new_p].0 = parents[up].0 + parents[vp].0;
        parents[new_p].1 = parents[up].1 + parents[vp].1;

        results.push(e_city);
    }

    results.pop();

    results.reverse();

    println!(
        "{}",
        results
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    );
}

struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if x == self.par[x] {
            x
        } else {
            self.par[x] = self.find(self.par[x]);
            self.par[x]
        }
    }
    fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
    fn unite(&mut self, a: usize, b: usize) {
        let aroot = self.find(a);
        let broot = self.find(b);
        if aroot == broot {
            return;
        }
        if self.rank[aroot] > self.rank[broot] {
            self.par[broot] = aroot;
        } else {
            self.par[aroot] = broot;
            if self.rank[aroot] == self.rank[broot] {
                self.rank[broot] += 1;
            }
        }
    }
}
