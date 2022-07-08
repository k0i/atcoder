use std::collections::HashMap;

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    a:[u64;n],
    q:usize
        }
    let mut m = HashMap::new();
    let mut sum = 0;
    for i in 0..n {
        let v = m.entry(a[i]).or_insert(0);
        *v += 1;
        sum += a[i];
    }
    for _ in 0..q {
        input! {
        b:u64,
        c:u64
        }
        let mut temp = 0;
        {
            let v = m.entry(b).or_insert(0);
            temp = *v;
            sum -= b * temp;
            *v = 0;
        }
        let v2 = m.entry(c).or_insert(0);
        *v2 += temp;
        sum += c * temp;
        println!("{:?}", sum);
    }
}
