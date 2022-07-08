use std::collections::HashSet;

use proconio::input;
pub fn main() {
    input! {
       n:usize,
      s:[u64;n]
    }
    let mut res = HashSet::new();
    for i in 0..n {
        if res.get(&s[i]).is_some() {
            res.remove(&s[i]);
        } else {
            res.insert(&s[i]);
        }
    }
    println!("{}", res.len());
}
