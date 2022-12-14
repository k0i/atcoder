use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::ops::{Add, Sub};

#[fastout]
pub fn main() {
    input! {
    n:usize,
    m:usize,
    ab:[(Usize1,Usize1);m]
    }
    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a].push(b);
    }
    let mut uf = UnionFindTree::new(n);
    let mut ans = 0;
    let mut res = vec![0];
    for i in (1..n).rev() {
        ans += 1;
        for &j in &graph[i] {
            if uf.is_same(i, j) {
                continue;
            }
            uf.unite(i, j);
            ans -= 1;
        }
        res.push(ans);
    }
    res.reverse();
    for i in 0..n {
        println!("{}", res[i]);
    }
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
fn d() {
    input! {
    s:Chars,
    k:i64
        }
    let mut cnt: Vec<i64> = vec![0; s.len() + 1];
    for i in 0..s.len() {
        if s[i] == '.' {
            cnt[i + 1] = cnt[i] + 1;
        } else {
            cnt[i + 1] = cnt[i];
        }
    }
    let mut ans = 0;
    let mut r = 0;
    for i in 0..s.len() {
        while r < s.len() && cnt[r + 1] - cnt[i] <= k {
            r += 1;
        }
        ans = std::cmp::max(ans, r - i);
    }
    println!("{:?}", ans);
}

/// Returns the cumulative sum.
/// ```
/// use k0i::cumsum::CumSum;
/// let x = vec![1, 2, 3,7];
/// let cumsum = x.cumsum();
/// assert_eq!(cumsum, [0, 1, 3, 6, 13]);
/// ```

pub trait CumSum<T> {
    fn cumsum(&self) -> Vec<T>;
}
impl<T> CumSum<T> for [T]
where
    T: PartialOrd + Copy + Default + Add<Output = T> + Sub<Output = T>,
{
    fn cumsum(&self) -> Vec<T> {
        let mut sec: Vec<T> = vec![T::default(); self.len() + 1];
        for i in 0..self.len() {
            sec[i + 1] = sec[i] + self[i];
        }
        sec
    }
}
