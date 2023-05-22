use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    b()
}
fn b() {
    input! {
    n:usize,
    a:[Usize1;n]
        }
    let mut visited = vec![false; n];
    visited[0] = true;
    let mut queue = vec![0];
    let mut ans = 0;
    while queue.len() > 0 {
        let current = queue.pop().unwrap();
        let next = a[current];
        if visited[next] {
            println!("-1");
            return;
        }
        visited[next] = true;
        ans += 1;
        queue.push(next);
        if next == 1 {
            println!("{}", ans);
            return;
        }
    }
    println!("{}", ans);
}

fn c() {
    input! {
    mut n:usize,
    mut m:usize
        }
    if n < m {
        std::mem::swap(&mut n, &mut m);
    }

    if n - m > 1 {
        println!("0");
        return;
    }

    let mo = 1000000007;

    let mut ans = 1;
    for i in 1..=n {
        ans *= i;
        ans %= mo;
    }
    for i in 1..=m {
        ans *= i;
        ans %= mo;
    }
    if m == n - 1 {
        println!("{}", ans % mo);
        return;
    } else {
        ans *= 2;
        ans %= mo;
        println!("{}", ans % mo);
    }
}
