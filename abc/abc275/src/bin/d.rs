use std::collections::HashMap;

use proconio::{fastout, input};
#[fastout]
pub fn main() {
    input! {n:u64}
    let mut memo = HashMap::new();
    println!("{}", f(n, &mut memo));
}

fn f(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if n == 0 {
        return 1;
    }
    if memo.contains_key(&n) {
        return memo[&n];
    }
    let ans = f(n / 2, memo) + f(n / 3, memo);
    memo.insert(n, ans);
    ans
}
