#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    mut m:usize,
    mut a:[(u64,u64);n],
        }

    a.sort_by_key(|x| x.0);
    let mut res = 0;
    let mut temp = 0;
    loop {
        if m == 0 {
            break;
        }
        if a[temp].1 > 0 {
            res += a[temp].0;
            m -= 1;
            a[temp].1 -= 1;
        } else {
            temp += 1;
        }
    }
    println!("{}", res);
}
