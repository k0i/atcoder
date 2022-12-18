use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};
#[fastout]
pub fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }
    let mut uf = UnionFindTree::new(n * 2);
    for &(u, v) in &uv {
        uf.unite(u, v + n);
        uf.unite(u + n, v);
    }
    let mut a = vec![0; 2 * n];
    for i in 0..(2 * n) {
        a[i] = uf.root(i);
    }
    println!("{:?}", a);
    for i in 0..n {
        if a[i] == a[i + n] {
            println!("{}", 0);
            return;
        }
    }

    let mut map = HashMap::<usize, Vec<_>>::new();
    for i in 0..n {
        let j = a[i];
        if let Some(x) = map.get_mut(&j) {
            x.push(i);
        } else {
            map.insert(j, vec![i]);
        }
    }

    let mut a0 = Vec::new();
    for (_, v) in &map {
        a0.push(v.len());
    }
    let mut count = n * (n - 1) / 2 - m;
    for &x in &a0 {
        count -= x * (x - 1) / 2;
    }
    println!("{}", count);
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
