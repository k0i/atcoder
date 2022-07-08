#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    mut a:[i64;n],
    q:usize,
    b:[i64;q],
        }
    a.sort_unstable();
    for i in b {
        let cnt = match a.binary_search(&i) {
            Ok(x) => x,
            Err(x) => {
                if x == 0 {
                    0
                } else {
                    x - 1
                }
            }
        };
        if a.len() == 1 {
            println!("{}", (a[0] - i).abs());
        } else if cnt == a.len() - 1 {
            println!("{}", (a[cnt] - i).abs());
        } else {
            println!(
                "{}",
                std::cmp::min((a[cnt] - i).abs(), (a[cnt + 1] - i).abs())
            );
        }
    }
}
