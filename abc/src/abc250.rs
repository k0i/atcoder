#[allow(unused_imports, unsafe_code, unused_unsafe)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    input! {
    n:u128
    }
    let mut prime = vec![true; 1000001];
    prime[0] = false;
    prime[1] = false;
    for i in 2..1000001 {
        if prime[i] {
            for j in 2.. {
                if i * j >= 1000000 {
                    break;
                }
                prime[i * j] = false;
            }
        }
    }
    let mut ans = 0;
    let mut primes = vec![];
    for i in 2..1000001 {
        if prime[i] {
            primes.push(i as u128);
        }
    }
    'outer: for i in 0..primes.len() {
        for j in i..primes.len() {
            if i == j {
                continue;
            }
            if primes[i] > n
                || primes[j] > n
                || primes[i] * primes[j] > n
                || primes[i] * primes[j] * primes[j] > n
                || primes[i] * primes[j] * primes[j] * primes[j] > n
            {
                continue 'outer;
            }
            ans += 1;
        }
    }
    println!("{}", ans);
}
fn c() {
    input! {
    n: usize,
    q: usize,
    x:[usize;q],
    }
    let mut ball: Vec<_> = (1..=n).collect();
    let mut order: Vec<_> = (1..=n).collect();
    for i in 0..q {
        let mut pos = order[x[i] - 1];
        let pos1 = order[x[i] - 1];
        if pos != n {
            pos += 1
        } else {
            pos -= 1
        }
        order.swap(ball[pos - 1] - 1, ball[pos1 - 1] - 1);
        ball.swap(pos - 1, pos1 - 1);
    }
    for i in ball {
        print!("{} ", i);
    }
}

pub fn b() {
    #[allow(unused_imports)]
    use proconio::{
        fastout, input,
        marker::{Bytes, Chars, Isize1, Usize1},
    };
    #[fastout]
    pub fn main() {
        input! {
            n: usize,
            a:usize,
            b:usize,
        }
        for i in 0..n {
            for j in 0..a {
                for k in 0..n {
                    for l in 0..b {
                        if k % 2 == 0 {
                            if i % 2 == 0 {
                                print!(".");
                            } else {
                                print!("#");
                            }
                        } else {
                            if i % 2 == 0 {
                                print!("#");
                            } else {
                                print!(".");
                            }
                        }
                    }
                }
                println!();
            }
        }
    }
}

pub fn a() {
    input! {
    h:usize,
    w:usize,
    r:usize,
    c:usize,
        }
    let mut ans = 4;
    if r - 1 == 0 {
        ans -= 1;
    }
    if c - 1 == 0 {
        ans -= 1;
    }
    if r + 1 > h {
        ans -= 1;
    }
    if c + 1 > w {
        ans -= 1;
    }
    println!("{}", ans);
}
