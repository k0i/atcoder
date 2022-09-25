use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    mut a:usize,
    mut b:usize,
    mut k:usize
        }
    let mut ans = String::new();
    while k > 0 {
        let comb = combinations(a + b - 1, a - 1);
        if k <= comb {
            ans.push('a');
            a -= 1;
        } else {
            ans.push('b');
            b -= 1;
            k -= comb;
        }
        if a == 0 {
            ans.push_str(&"b".repeat(b));
            break;
        }
        if b == 0 {
            ans.push_str(&"a".repeat(a));
            break;
        }
    }
    println!("{}", ans);
}

fn combinations(a: usize, b: usize) -> usize {
    let mut res = 1;
    for i in 0..b {
        res *= a - i;
        res /= i + 1;
    }
    res
}
