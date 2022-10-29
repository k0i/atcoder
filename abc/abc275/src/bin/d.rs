use std::collections::{BTreeSet, HashMap};

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {n:u64}
    let mut memo = HashMap::new();
    memo.insert(0, 1);
    memo.insert(1, 2);
    memo.insert(2, 3);
    memo.insert(3, 4);
    println!("{}", f(n, &mut memo));
}

fn f(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    if memo.contains_key(&n) {
        return memo[&n];
    }
    let ans = f(n / 2, memo) + f(n / 3, memo);
    memo.insert(n, ans);
    f(n / 2, memo) + f(n / 3, memo)
}
