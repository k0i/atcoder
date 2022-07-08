use std::collections::{VecDeque, HashSet};

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    input! {
        n:usize,
        t:[(usize,usize);n-1]
    }
    let mut a = vec![vec![]; n];
    for edge in t {
        a[(edge.0) - 1].push(edge.1-1);
        a[(edge.1) - 1].push(edge.0-1);
    }
    let mut visited = vec![false; n];
let mut colors = vec![false;n];
    let mut res1 =HashSet::new();
    let mut res2 = HashSet::new();
    let mut stack = VecDeque::new();
    stack.push_front(0);
    while stack.len() > 0 {
        let current = stack.pop_front().unwrap();
        if visited[current] {
            continue;
        }
        visited[current] = true;
        let next = a[current].clone();
        let c = colors[current];
        next.into_iter().for_each(|x| {
            colors[x]=!c;
            if c {
                res1.insert(x+1);
            } else {
                res2.insert(x+1);
            };
            stack.push_front(x)
        });
    }

    if res1.len() >= n / 2 {
       res1.iter().take(n/2).for_each(move |x| print!("{} ", x));
    } else {
       res2.iter().take(n/2).for_each(move |x| print!("{} ", x));
    }
}
