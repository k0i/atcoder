#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
       n:usize,
       p:u64,
       a:[u64;n],
    }
    let even = a.iter().filter(|&x| x % 2 == 0).collect::<Vec<_>>().len();
    let odd = n - even;
    if p == 1 && odd == 0 {
        println!("{}", 0);
    } else if n == even {
        println!("{}", 2_u64.pow(n as u32));
    } else {
        println!("{:?}", 2_u64.pow((n - 1) as u32));
    }
}
