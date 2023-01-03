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
    ab:[(Usize1,Usize1);m],
        }
    let mut tree = vec![vec![]; n];
    for i in 0..m {
        let (a, b) = ab[i];
        tree[a].push(b);
        tree[b].push(a);
    }
    let mut dist = vec![0; n];
    let mut shortest_path_cnt = vec![0; n];
    shortest_path_cnt[0] = 1;
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(0);
    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        for &u in &tree[v] {
            if dist[u] == 0 {
                dist[u] = dist[v] + 1;
                shortest_path_cnt[u] = shortest_path_cnt[v];
                queue.push_back(u);
            } else if dist[u] == dist[v] + 1 {
                shortest_path_cnt[u] += shortest_path_cnt[v];
                shortest_path_cnt[u] %= 1_000_000_007;
            }
        }
    }
    println!("{}", shortest_path_cnt[n - 1]);
}
fn c() {
    input! {
    s:Chars
        }
    let ch = ['c', 'h', 'o', 'k', 'u', 'd', 'a', 'i'];
    let mut dp = vec![vec![0; ch.len() + 1]; s.len() + 1];
    dp[0][0] = 1;
    for i in 0..s.len() {
        for j in 0..=ch.len() {
            if j < ch.len() && s[i] == ch[j] {
                dp[i + 1][j + 1] += dp[i][j];
                dp[i + 1][j + 1] %= 1000000007;
            }
            dp[i + 1][j] += dp[i][j];
            dp[i + 1][j] %= 1000000007;
        }
    }
    println!("{}", dp[s.len()][ch.len()]);
    println!("{:?}", dp);
}
