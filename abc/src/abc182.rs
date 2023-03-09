#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    e()
}

fn e() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        m: usize,
        p: [(usize, usize); n],
        q: [(usize, usize); m]
    }

    let mut res = vec![vec![0; w]; h];
    for (a, b) in p {
        res[a - 1][b - 1] = 1;
    }
    for (c, d) in q {
        res[c - 1][d - 1] = -1;
    }
    let mut res2 = res.clone();
    for i in 0..h {
        for j in 1..w {
            if res[i][j] >= 0 && res[i][j - 1] >= 0 {
                res[i][j] += res[i][j - 1];
            }
        }
        for j in (0..w - 1).rev() {
            if res[i][j] >= 0 && res[i][j + 1] >= 0 {
                res[i][j] += res[i][j + 1];
            }
        }
    }
    for j in 0..w {
        for i in 1..h {
            if res2[i][j] >= 0 && res2[i - 1][j] >= 0 {
                res2[i][j] += res2[i - 1][j];
            }
        }
        for i in (0..h - 1).rev() {
            if res2[i][j] >= 0 && res2[i + 1][j] >= 0 {
                res2[i][j] += res2[i + 1][j];
            }
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if res[i][j] > 0 || res2[i][j] > 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

fn d() {
    input! {
        n: usize,
        a: [i128; n],
    }

    let mut cm = vec![0; n + 1];
    for i in 0..n {
        cm[i + 1] = cm[i] + a[i];
    }

    let mut max = vec![0; n + 2];
    for i in 0..n + 1 {
        max[i + 1] = std::cmp::max(max[i], cm[i]);
    }
    let mut ans = 0;
    let mut cur = 0;
    for i in 0..n + 1 {
        ans = std::cmp::max(ans, cur + max[i + 1]);
        cur += cm[i];
    }

    println!("{}", ans);
}
fn c() {
    input! {n:Chars}
    let m: Vec<u64> = n.iter().map(|x| x.to_digit(10).unwrap() as u64).collect();
    let sum: u64 = m.iter().sum();
    if sum % 3 == 0 {
        println!("0");
        return;
    }
    let mut modulo_one = 0;
    let mut modulo_two = 0;
    for i in m.clone() {
        if i % 3 == 1 {
            modulo_one += 1;
        } else if i % 3 == 2 {
            modulo_two += 1;
        }
    }
    if sum % 3 == 1 {
        if modulo_one >= 1 && m.len() > 1 {
            println!("1");
        } else if modulo_two >= 2 && m.len() > 2 {
            println!("2");
        } else {
            println!("-1");
        }
        return;
    }
    if modulo_two >= 1 && m.len() > 1 {
        println!("1");
    } else if modulo_one >= 2 && m.len() > 2 {
        println!("2");
    } else {
        println!("-1");
    }
}
