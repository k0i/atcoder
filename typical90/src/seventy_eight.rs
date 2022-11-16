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
    ab:[(Usize1,Usize1);m]
        }
    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }
    let mut ans = vec![];
    for i in 0..n {
        let mut cnt = 0;
        for j in 0..graph[i].len() {
            if graph[i][j] < i {
                cnt += 1;
            }
        }
        if cnt == 1 {
            ans.push(i + 1);
        }
    }
    println!("{:?}", ans.len());
}
