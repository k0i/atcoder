use std::collections::HashMap;

use proconio::{fastout, input, marker::Chars};
#[fastout]
pub fn main() {
    input! {
    h:usize,
    w:usize,
    g:[Chars;h]
        }
    let mut start = (0, 0);
    let mut uf = UnionFind::new(h * w);
    let dx = vec![-1, 0, 1, 0];
    let dy = vec![0, -1, 0, 1];
    for i in 0..h {
        for j in 0..w {
            if g[i][j] == 'S' {
                start = (i, j);
                continue;
            }
            if g[i][j] == '#' {
                continue;
            }
            for k in 0..2 {
                let nx = j as isize + dx[k];
                let ny = i as isize + dy[k];
                if nx < 0 || ny < 0 || nx >= w as isize || ny >= h as isize {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if g[ny][nx] == '.' {
                    uf.unite(i * w + j, ny * w + nx);
                }
            }
        }
    }
    let mut cnt = HashMap::new();
    for i in 0..4 {
        let nx = start.1 as isize + dx[i];
        let ny = start.0 as isize + dy[i];
        if nx < 0 || ny < 0 || nx >= w as isize || ny >= h as isize {
            continue;
        }
        let v = cnt
            .entry(uf.root(ny as usize * w + nx as usize))
            .or_insert(0);
        *v += 1;
    }
    for (_, v) in cnt.into_iter() {
        if v >= 2 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

#[derive(Debug, Clone)]
struct UnionFind {
    par: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            par: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.par[x] = self.root(self.par[x]);
            self.par[x]
        }
    }
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    fn size(&mut self, x: usize) -> usize {
        let r = self.root(x);
        self.size[r]
    }
    fn unite(&mut self, x: usize, y: usize) {
        let x = self.root(x);
        let y = self.root(y);
        if x == y {
            return;
        }
        if self.size[x] < self.size[y] {
            self.par[x] = y;
            self.size[y] += self.size[x];
        } else {
            self.par[y] = x;
            self.size[x] += self.size[y];
        }
    }
}
