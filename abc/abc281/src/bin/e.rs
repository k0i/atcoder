use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

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
    k:usize,
    a:[usize;n],
        }

    let mut window = a.clone().into_iter().take(m).collect::<Vec<_>>();
    let mut idx_hash = HashMap::new();
    for i in 0..m {
        idx_hash.insert(i, window[i]);
    }
    window.sort();
    let mut sum_of_k = window.iter().take(k).sum::<usize>();
    let mut sorted_idx_hash = HashMap::new();
    for i in 0..k {
        sorted_idx_hash.insert(window[i], i);
    }
    print!("{} ", sum_of_k);
    for i in 0..n - m {
        let v = idx_hash.remove(&i).unwrap();
        sum_of_k -= v;
        let idx = sorted_idx_hash.remove(&v).unwrap_or(1000000);
        if idx != 1000000 {
            window.remove(idx);
        }
        let v = a[i + m];
        let mut left = 0;
        let mut right = k - 1;
        while left < right {
            let mid = (left + right) / 2;
            if window[mid] < v {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        window.insert(right + 1, v);
        sorted_idx_hash.insert(v, right + 1);
        idx_hash.insert(i + m, v);
        if left < k {
            sum_of_k += v;
        } else {
            sum_of_k += window[k - 1];
        }
        println!("{:?}", window);

        if i == n - m - 1 {
            println!("{}", sum_of_k);
        } else {
            print!("{} ", sum_of_k);
        }
    }
}
#[fastout]
fn main() {
    input! {
        n:usize,m:usize,k:usize,
        a:[i64;n]
    }
    let mut p = std::collections::BTreeSet::new();
    let mut q = std::collections::BTreeSet::new();
    let mut s = 0;
    for i in 0..n {
        q.insert((a[i], i));
        if i >= m {
            if p.contains(&(a[i - m], i - m)) {
                p.remove(&(a[i - m], i - m));
                s -= a[i - m];
            } else {
                q.remove(&(a[i - m], i - m));
            }
        }
        let x = *q.iter().next().unwrap();
        q.remove(&x);
        p.insert(x);
        s += x.0;
        if p.len() > k {
            let y = *p.iter().next_back().unwrap();
            p.remove(&y);
            s -= y.0;
            q.insert(y);
        }
        if i >= m - 1 {
            print!("{} ", s)
        }
    }
}
