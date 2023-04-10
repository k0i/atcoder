#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};
#[fastout]
pub fn main() {
    d()
}

fn d() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m]
    }

    let mut ans;
    if m == 0 {
        ans = 1;
    } else {
        a.sort_by_key(|&x| Reverse(x));
        let mut k = n - a[0];
        let mut temp = a[0];
        let mut white = Vec::new();
        white.push(k);
        for i in a.iter().skip(1) {
            white.push(temp - i - 1);
            if k == 0 {
                k = temp - i - 1;
            } else if temp - i - 1 != 0 {
                k = k.min(temp - i - 1);
            }
            temp = *i;
        }
        if temp != 1 {
            k = k.min(temp - 1);
            white.push(temp - 1);
        }
        ans = 0;
        for i in white {
            if i == 0 {
                continue;
            }
            ans += (i + k - 1) / k;
        }
    }

    println!("{}", ans);
}
fn f() {
    input! {
        n :usize,
        q: usize,
        a: [usize; n],
        xy: [(usize, usize, usize); q]
    }

    let mut tree = SegmentTree::new(n);

    for i in 0..n {
        tree.update(i, a[i] as isize);
    }

    for (t, x, y) in xy {
        if t == 1 {
            let value = tree.query(x - 1, x);

            tree.update(x - 1, value ^ y as isize);
        } else {
            println!("{}", tree.query(x - 1, y));
        }
    }
}

struct SegmentTree {
    n: usize,
    data: Vec<isize>,
}

impl SegmentTree {
    fn new(size: usize) -> Self {
        let mut n = 1;
        while n < size {
            n *= 2;
        }
        Self {
            n,
            data: vec![0; 2 * n - 1],
        }
    }

    fn update(&mut self, k: usize, a: isize) {
        let mut i = k + self.n - 1;
        self.data[i] = a;

        while i > 0 {
            i = (i - 1) / 2;
            self.data[i] = self.data[i * 2 + 1] ^ self.data[i * 2 + 2];
        }
    }
    fn query(&self, a: usize, b: usize) -> isize {
        self.query_inner(a, b, 0, 0, self.n)
    }

    fn query_inner(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> isize {
        if r <= a || b <= l {
            return 0;
        }

        if a <= l && r <= b {
            return self.data[k];
        }
        let v_l = self.query_inner(a, b, k * 2 + 1, l, (l + r) / 2);
        let v_r = self.query_inner(a, b, k * 2 + 2, (l + r) / 2, r);

        v_l ^ v_r
    }
}
