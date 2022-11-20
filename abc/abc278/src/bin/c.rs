use proconio::{fastout, input};
use std::collections::HashMap;
#[fastout]
pub fn main() {
    input! {
    _n:usize,
    q:usize,
        }
    let mut trees = HashMap::new();
    for i in 0..q {
        input! {t:usize}

        match t {
            1 => {
                input! {a:usize,b:usize}
                trees.entry(a).or_insert_with(HashMap::new).insert(b, i);
            }
            2 => {
                input! {a:usize,b:usize}
                if let Some(x) = trees.get_mut(&a) {
                    x.remove(&b);
                }
            }
            _ => {
                input! {a:usize,b:usize}
                let mut ans = "No";
                if let Some(x) = trees.get(&a) {
                    if x.get(&b).is_some() {
                        if let Some(z) = trees.get(&b) {
                            if z.get(&a).is_some() {
                                ans = "Yes";
                            }
                        }
                    }
                }
                println!("{}", ans);
            }
        }
    }
}
