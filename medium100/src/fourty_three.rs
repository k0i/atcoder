#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
       x:u64
    }
    if x == 1 {
        println!("1")
    } else if x == 2 {
        println!("2")
    } else {
        let mut temp = 0;
        let mut res = 0;
        for i in 1..x + 1 {
            if temp >= x {
                res = i - 1;
                break;
            }
            temp += i
        }
        println!("{:?}", res);
    }
}
