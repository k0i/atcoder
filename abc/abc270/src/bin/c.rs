use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        n:usize,
        start:usize,
        mut goal:usize,
        cd:[(usize,usize);n-1]
    }
    let mut connect = vec![vec![]; n + 1];

    for i in 0..n - 1 {
        let (c, d) = cd[i];
        connect[c].push(d);
        connect[d].push(c);
    }

    let mut queue = std::collections::VecDeque::new();
    let mut prev = vec![std::i64::MAX; n + 1];
    prev[start] = -1;
    queue.push_back(start);
    while let Some(now) = queue.pop_front() {
        for &next in connect[now].iter() {
            if prev[next] != std::i64::MAX {
                continue;
            }
            prev[next] = now as i64;
            queue.push_back(next);
        }
    }
    let mut res = vec![];
    while (goal != start) {
        res.push(goal);
        goal = prev[goal] as usize;
    }
    res.reverse();
    print!("{} ", start);
    for i in 0..res.len() {
        if i == res.len() - 1 {
            print!("{}", res[i]);
        } else {
            print!("{} ", res[i]);
        }
    }
}
