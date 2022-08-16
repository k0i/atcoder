use proconio::{input, marker::Chars};
use std::collections::HashMap;
pub fn main() {
    d()
}

pub fn d() {
    input! {n:usize,
    a:[u64;n]
        }
    let mut cnt = HashMap::new();
    for i in a {
        let v = cnt.entry(i).or_insert(0i64);
        *v += 1;
    }
    let mut res = 0;

    for (k, v) in cnt.iter() {
        let mut div = vec![];
        let mut i = 1;
        while i * i <= *k {
            if k % i == 0 {
                div.push(i);
                if i != k / i {
                    div.push(k / i);
                }
            }
            i += 1;
        }
        for d in div {
            res += v * cnt.get(&(k / d)).unwrap_or(&0) * cnt.get(&d).unwrap_or(&0);
        }
    }
    println!("{:?}", res);
}
pub fn c() {
    input! {
    n:usize,
    k:usize,
    s:[String;n]
        }
    let mut res = 0;
    for i in 0..1 << n {
        let mut ss = Vec::new();
        for j in 0..n {
            if i & (1 << j) != 0 {
                let m = s[j].clone();
                ss.push(m.chars().collect::<Vec<char>>());
            }
        }
        let mut count = HashMap::new();
        for s in ss {
            for c in s {
                *count.entry(c).or_insert(0) += 1;
            }
        }
        let max = count.iter().filter(|&(_, &v)| v == k).count();
        if max > res {
            res = max;
        }
    }
    println!("{}", res);
}

pub fn b() {
    use std::collections::HashSet;
    fn main() {
        input! {
        s:Chars
        }
        let mut upper = false;
        let mut smaller = false;
        let mut dup = false;
        let mut unique = HashSet::new();
        for i in s {
            if i.is_uppercase() {
                upper = true;
            } else {
                smaller = true;
            }
            if unique.contains(&i) {
                dup = true;
                break;
            } else {
                unique.insert(i);
            }
        }
        if upper && !dup && smaller {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

pub fn a() {
    input! {
    a:u64,
    b:u64,
    c:u64,
    d:u64,
    e:u64,
    f:u64,
    x:u64
        }

    let tak = solve(a, b, c, x);
    let aoki = solve(d, e, f, x);
    match tak.cmp(&aoki) {
        std::cmp::Ordering::Less => println!("Aoki"),
        std::cmp::Ordering::Greater => println!("Takahashi"),
        std::cmp::Ordering::Equal => println!("Draw"),
    }
}

fn solve(a: u64, b: u64, c: u64, x: u64) -> u64 {
    let cycle = x / (a + c);
    let rest = x % (a + c);
    (cycle * a + std::cmp::min(rest, a)) * b
}
