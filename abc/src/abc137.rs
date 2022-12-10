use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    mut s:[Chars;n]
    }
    let mut ss = vec![];
    for i in 0..n {
        s[i].sort();
        ss.push(s[i].iter().collect::<String>());
    }
    ss.sort();
    let mut hash = std::collections::HashMap::new();
    for i in ss {
        let mut count = hash.entry(i).or_insert(0);
        *count += 1;
    }
    let mut ans: u64 = 0;
    for i in hash {
        ans += i.1 * (i.1 - 1) / 2;
    }
    println!("{}", ans);
}
