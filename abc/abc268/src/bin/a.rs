use im_rc::HashSet;
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {a:usize,b:usize,c:usize,d:usize,e:usize}
    let mut f = HashSet::new();
    f.insert(a);
    f.insert(b);
    f.insert(c);
    f.insert(d);
    f.insert(e);
    println!("{:?}", f.len());
}
