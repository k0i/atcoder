#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    input! {
      s: Chars,
    }
    let len = s.len();
    let mut cnt = vec![0; 10];
    for c in s {
        cnt[(c as u8 - '0' as u8) as usize] += 1;
    }
    let mut ans = false;
    for x in (0..1000).step_by(8) {
        let s1 = x.to_string();
        if !((len <= 3 && len == s1.len()) || (len > 3 && s1.len() == 3)) {
            continue;
        }
        let mut cnt1 = vec![0; 10];
        for c1 in s1.chars() {
            cnt1[(c1 as u8 - '0' as u8) as usize] += 1;
        }
        let flag = (0..10).all(|i| cnt1[i] <= cnt[i]);
        ans |= flag;
    }
    println!("{}", if ans { "Yes" } else { "No" });
}

fn c() {
    input! {n:usize,p:[(i64,i64);n]}
    let mut res = "No";
    'outer: for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if (p[k].1 - p[i].1) * (p[j].0 - p[i].0) == (p[j].1 - p[i].1) * (p[k].0 - p[i].0) {
                    res = "Yes";
                    break 'outer;
                }
            }
        }
    }
    println!("{}", res);
}
