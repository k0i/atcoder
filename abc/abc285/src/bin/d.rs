use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};
pub fn main() {
    input! {
    n:usize,
        }
    let mut tr = HashMap::new();
    let mut before = HashSet::new();
    let mut after = HashSet::new();

    for i in 0..n {
        input! {
        s:String,
        t:String,
        }
        before.insert(s.clone());
        after.insert(t.clone());
        tr.insert(t, s);
    }
    let diff = after.difference(&before).collect_vec();
    let mut cnt = 0;
    for i in diff {
        let bf = tr.get(i).unwrap();
        if !before.contains(bf) {
            cnt += 1;
            continue;
        }
        let mut tmp = 1;
        let mut tmp_c = bf;
        while tr.contains_key(tmp_c) {
            tmp += 1;
            tmp_c = tr.get(tmp_c).unwrap();
        }
        cnt += tmp;
    }
    if cnt == n {
        println!("Yes");
    } else {
        println!("No");
    }
}

/// Given the following tree structure
///         0              7       9
///        / \            /       / \
///       1   3          8       10  11
///      /   / \                      \
///     2   4   5                      12
///    /
///   6
/// ```
/// use k0i::union_find::UnionFindTree;
/// let tree = vec![(0,1),(0,3),(1,2),(3,4),(3,5),(2,6),(7,8),(9,10),(9,11),(11,12)];
/// let mut uft = UnionFindTree::new(13);
/// for (a,b) in tree{
///     uft.unite(a,b);
/// }
/// assert!(uft.is_same(6,5));
/// assert!(uft.is_same(6,4));
/// assert!(uft.is_same(0,4));
/// assert!(uft.is_same(2,5));
/// assert!(uft.is_same(7,8));
/// assert!(uft.is_same(9,11));
///
/// assert_eq!(uft.is_same(0,9),false);
/// assert_eq!(uft.is_same(7,12),false);
///
/// assert_eq!(uft.size(0),7);
/// assert_eq!(uft.size(7),2);
/// assert_eq!(uft.size(9),4);
/// ```

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
