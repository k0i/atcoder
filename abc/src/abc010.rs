use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    start:(i64,i64),
    end:(i64,i64),
    t:i64,
    v:i64,
    n:usize,
    points:[(i64,i64);n],
        }
    let dis = t * v;
    for i in 0..n {
        let p = points[i];
        if (((p.0 - start.0).pow(2) + (p.1 - start.1).pow(2)) as f64).sqrt()
            + (((p.0 - end.0).pow(2) + (p.1 - end.1).pow(2)) as f64).sqrt()
            <= dis as f64
        {
            println!("YES");
            return;
        }
    }
    println!("NO");
}
