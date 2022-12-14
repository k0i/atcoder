use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    m:usize,
    }
    let mut abc = vec![];
    let mut ans = 0;
    for i in 0..m {
        input! {
        a:usize,b:usize,
        c:i64
        }
        abc.push((c, a, b));
        if c > 0 {
            ans += c;
        }
    }
    abc.sort();
    let mut uf = UnionFindTree::new(n + 1);
    for (c, a, b) in abc {
        if c <= 0 {
            uf.unite(a, b);
            continue;
        }
        if !uf.is_same(a, b) {
            uf.unite(a, b);
            ans -= c;
        }
    }
    println!("{}", ans);
}

#[derive(Debug)]
pub struct UnionFindTree {
    parents: Vec<isize>,
    size: Vec<usize>,
}

impl UnionFindTree {
    pub fn new(n: usize) -> Self {
        Self {
            parents: vec![-1; n],
            size: vec![1; n],
        }
    }
    pub fn root(&mut self, x: usize) -> usize {
        if self.parents[x] == -1 {
            x
        } else {
            self.parents[x] = self.root(self.parents[x] as usize) as isize;
            self.parents[x] as usize
        }
    }

    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.root(x);
        let mut y = self.root(y);

        if x == y {
            return false;
        }

        if self.size[x] < self.size[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parents[y] = x as isize;
        self.size[x] += self.size[y];
        true
    }

    pub fn size(&mut self, x: usize) -> usize {
        let r = self.root(x);
        self.size[r]
    }
}
