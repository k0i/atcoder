use std::f64::consts::PI;

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    start:(i64,i64),
    end:(i64,i64),
    circles:[(i64,i64,i64);n],
        }
    let start_circle = on_circle(start, &circles);
    let end_circle = on_circle(end, &circles);
    if start_circle.is_none() || end_circle.is_none() {
        println!("No");
        return;
    }
    let mut tree = vec![vec![]; n];

    for i in 0..n {
        for j in i + 1..n {
            if is_cross(circles[i], circles[j]) {
                tree[i].push(j);
                tree[j].push(i);
            }
        }
    }
    let mut queue = vec![];
    let mut visited = vec![false; n];
    queue.push(start_circle.unwrap());
    while !queue.is_empty() {
        let now = queue.pop().unwrap();
        if now == end_circle.unwrap() {
            println!("Yes");
            return;
        }
        visited[now] = true;
        for &next in &tree[now] {
            if !visited[next] {
                queue.push(next);
            }
        }
    }
    println!("No");
}

fn on_circle(p: (i64, i64), circles: &[(i64, i64, i64)]) -> Option<usize> {
    for i in 0..circles.len() {
        let (x, y, r) = circles[i];
        if (x - p.0).pow(2) + (y - p.1).pow(2) == r.pow(2) {
            return Some(i);
        }
    }
    None
}

fn is_cross(c1: (i64, i64, i64), c2: (i64, i64, i64)) -> bool {
    let (x1, y1, r1) = c1;
    let (x2, y2, r2) = c2;
    let d_pow = (x1 - x2).pow(2) + (y1 - y2).pow(2);
    if d_pow > (r1 + r2).pow(2) {
        return false;
    }
    if d_pow < (r1 - r2).pow(2) {
        return false;
    }
    true
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
