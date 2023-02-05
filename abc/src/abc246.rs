use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    input! {
    n:usize
    }
    let f = |a: usize, b: usize| -> usize { a * a * a + a * a * b + a * b * b + b * b * b };
    let mut ans = std::usize::MAX;
    for a in 0..=1000001 {
        let mut left = -1;
        let mut right = 1000001;
        while right - left > 1 {
            let mid = (left + right) / 2;
            if f(a, mid as usize) < n {
                left = mid;
            } else {
                right = mid;
            }
        }
        ans = ans.min(f(a, right as usize));
    }
    println!("{}", ans);
}
fn c() {
    input! {
     n:usize,
    mut k:usize,
     x:usize,
         mut a:[usize;n]
         }

    a.sort();
    a.reverse();
    for i in 0..n {
        if a[i] < x {
            break;
        }
        let y = a[i] / x;
        if k >= y {
            a[i] -= x * y;
            k -= y;
        } else {
            a[i] -= x * k;
            println!("{}", a.iter().sum::<usize>());
            return;
        }
        if k == 0 {
            println!("{}", a.iter().sum::<usize>());
            return;
        }
    }
    a.sort();
    a.reverse();
    for i in 0..n {
        a[i] = 0;
        k -= 1;
        if k == 0 {
            break;
        }
    }
    println!("{}", a.iter().sum::<usize>());
}
