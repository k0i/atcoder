use std::collections::HashMap;

use proconio::input;
pub fn main() {
    input! {
       n:usize,
       s:[u64;n]
    }
    let mut res: HashMap<u64, u64> = HashMap::new();
    for i in s {
        if i > 0 {
            res.entry(i - 1).and_modify(|x| *x += 1).or_insert(1);
        }
        res.entry(i).and_modify(|x| *x += 1).or_insert(1);

        res.entry(i + 1).and_modify(|x| *x += 1).or_insert(1);
    }
    println!("{}", res.values().max().unwrap());
}
