use std::collections::hash_set::Union;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    m:usize,
    f:[(usize,usize);m]
        }
    let mut uf = UnionFindTree::new(n);
    for (a, b) in f {
        uf.unite(a - 1, b - 1);
    }
    println!("{}", uf.max_size());
}

fn c() {
    input! {
    n:usize,
    a:[u128;n]
        }
    let modn = 10_u128.pow(9) + 7;
    let mut res = 0;
    let mut a_sum = a.iter().sum::<u128>();
    for i in 0..n {
        a_sum -= a[i];
        res += a[i] * a_sum;
        res %= modn;
    }
    println!("{}", res % modn);
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
    pub fn max_size(&self) -> usize {
        let mut mx = 1;
        for i in self.size.iter() {
            mx = std::cmp::max(mx, *i);
        }
        mx
    }
}
