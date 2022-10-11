use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    m:i64
    }
    let mut d = vec![];
    let mut a = -1001;
    loop {
        let mut b = -1001;
        while b <= 1000 {
            if a * a + b * b == m {
                d.push((a, b));
            }
            b += 1;
        }
        a += 1;
        if a > 1000 {
            break;
        }
    }
    let mut map = vec![vec![-1; n]; n];
    map[0][0] = 0;
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in d.iter() {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0 && nx < n as i64 && ny >= 0 && ny < n as i64 {
                if map[nx as usize][ny as usize] == -1 {
                    map[nx as usize][ny as usize] = map[x as usize][y as usize] + 1;
                    queue.push_back((nx, ny));
                }
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            print!("{} ", map[i][j]);
        }
        println!();
    }
}
fn c() {
    input! {
    n:usize,
    mut a:[i64;n]
        }
    let mut odd = vec![];
    let mut even = vec![];
    for i in a.clone() {
        if i % 2 == 0 {
            even.push(i);
        } else {
            odd.push(i);
        }
    }
    let mut res = -1;
    even.sort_by(|a, b| b.cmp(a));
    odd.sort_by(|a, b| b.cmp(a));

    if even.len() >= 2 {
        res = std::cmp::max(res, even[0] + even[1]);
    }
    if odd.len() >= 2 {
        res = std::cmp::max(res, odd[0] + odd[1]);
    }
    println!("{}", res);
}
