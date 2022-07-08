#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
 
use std::collections::HashSet;
 
fn search(s: Vec<char>, t: char) -> usize {
    if s.iter().all(|&q| q == t) {
        return 0;
    }
    let mut ss = vec![];
    for i in 0..s.len() - 1 {
        let c1 = s[i];
        let c2 = s[i + 1];
        if c1 == t || c2 == t {
            ss.push(t);
        } else {
            ss.push(c1);
        }
    }
 
    1 + search(ss, t)
}
 
#[fastout]
pub fn main() {
    input! {
        s:Chars,
    }
    let mut set = HashSet::new();
    for &i in &s {
        set.insert(i);
    }
    let mut ans = 1 << 30;
    for i in set {
        ans = std::cmp::min(ans, search(s.clone(), i));
    }
    println!("{}", ans);
}