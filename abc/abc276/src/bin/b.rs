use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    m:usize,
    ab:[(Usize1,Usize1);m],
        }

    let mut res = vec![vec![]; n];
    for i in 0..m {
        let (a, b) = ab[i];
        res[a].push(b);
        res[b].push(a);
    }
    for i in 0..n {
        res[i].sort();
    }

    for i in 0..n {
        let d = res[i].len();
        print!("{} ", d);
        let i: Vec<_> = res[i]
            .iter()
            .map(|x| {
                let y = x + 1;
                return y;
            })
            .collect();
        println!("{}", i.into_iter().join(" "));
    }
}
