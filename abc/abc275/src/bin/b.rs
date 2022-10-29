use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    mut  a:u128,
    mut  b:u128,
    mut  c:u128,
    mut  d:u128,
    mut  e:u128,
    mut  f:u128,
         }
    let m = 998244353;
    a %= m;
    b %= m;
    c %= m;
    d %= m;
    e %= m;
    f %= m;
    let abc = (a * b) % m * c % m;
    let def = (d * e) % m * f % m;
    println!("{}", (abc + m - def) % m);
}
