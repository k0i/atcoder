use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,k:usize,r:u64,s:u64,p:u64,t:Chars
        }

    let mut his = vec![];
    let mut res = 0;
    for i in 0..n {
        match t[i] {
            'r' => {
                if i < k {
                    res += p;
                    his.push('p');
                } else if k <= i && his[i - k] != 'p' {
                    res += p;
                    his.push('p');
                } else {
                    his.push('n');
                }
            }
            's' => {
                if i < k {
                    res += r;
                    his.push('r');
                } else if k <= i && his[i - k] != 'r' {
                    res += r;
                    his.push('r');
                } else {
                    his.push('n');
                }
            }
            'p' => {
                if i < k {
                    res += s;
                    his.push('s');
                } else if k <= i && his[i - k] != 's' {
                    res += s;
                    his.push('s');
                } else {
                    his.push('n');
                }
            }
            _ => {}
        }
    }
    println!("{:?}", res);
}
