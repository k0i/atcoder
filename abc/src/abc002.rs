use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
     mut a:(f64,f64),
    mut  b:(f64,f64),
     mut c:(f64,f64),
         }
    let x = a.0;
    let y = a.1;
    a = (0., 0.);
    b.0 -= x;
    b.1 -= y;
    c.0 -= x;
    c.1 -= y;
    let area = (1. / 2. * (b.0 * c.1 - b.1 * c.0)).abs();
    println!("{:?}", area);
}
