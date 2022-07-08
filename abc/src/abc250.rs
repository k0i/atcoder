#[allow(unused_imports, unsafe_code, unused_unsafe)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
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
