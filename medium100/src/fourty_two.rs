#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    c:Chars
    }
    let mut res = 0;
    for i in 0..c.len() {
      if c[i]=='U'{
         res+=c.len()+i-1
      }else{
         res += 2*c.len() - 2 - i
      }
    }
    println!("{}", res);
}
