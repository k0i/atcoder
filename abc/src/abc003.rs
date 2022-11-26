use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        s: String,
        t: String,
    }

    let len = s.len();
    let c = ['a', 't', 'c', 'o', 'd', 'e', 'r'];

    for i in 0..len {
        let si = s.chars().nth(i).unwrap();
        let ti = t.chars().nth(i).unwrap();
        if si == ti {
            continue;
        }
        if si == '@' && c.contains(&ti) {
            continue;
        }
        if c.contains(&si) && ti == '@' {
            continue;
        }
        println!("You will lose");
        return;
    }
    println!("You can win");
}
