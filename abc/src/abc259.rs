use std::f64::consts::PI;

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    c();
}

fn c() {
    input! {s:Chars,t:Chars}
    let mut lre_s = vec![];
    let mut lre_t = vec![];
    let mut cur = ('0', 0);
    for i in s {
        if i == cur.0 {
            cur.1 += 1;
            continue;
        }
        if cur.1 > 0 {
            lre_s.push(cur);
            cur = (i, 1);
            continue;
        }
        cur.0 = i;
        cur.1 = 1;
    }
    lre_s.push(cur);
    cur = ('0', 0);
    for i in t {
        if i == cur.0 {
            cur.1 += 1;
            continue;
        }
        if cur.1 > 0 {
            lre_t.push(cur);
            cur = (i, 1);
            continue;
        }
        cur.0 = i;
        cur.1 = 1;
    }
    lre_t.push(cur);

    if lre_s.len() != lre_t.len() {
        println!("No");
    } else {
        let mut ans = true;
        for i in 0..lre_s.len() {
            if lre_s[i].0 != lre_t[i].0 {
                ans = false;
                break;
            }
            if !(lre_s[i].1 == lre_t[i].1 || lre_s[i].1 < lre_t[i].1 && lre_s[i].1 >= 2) {
                ans = false;
            }
        }
        if ans {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn b() {
    input! {
    x:f64,y:f64,t:f64
        }
    let r = x.hypot(y);
    let mut theta = y.atan2(x);
    theta += t * PI / 180.0;
    println!("{} {}", theta.cos() * r, theta.sin() * r);
}

fn a() {
    input! {
    n:u64,
      mut m:i64,
      mut x:i64,
      mut t:i64,
      mut d:i64
    }
    let mut ans = t;
    if m < x {
        ans -= (x - m) * d
    }
    println!("{}", ans);
}
