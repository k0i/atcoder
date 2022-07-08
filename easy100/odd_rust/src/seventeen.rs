use proconio::input;
pub fn main() {
     input! {
        n:isize,
        mut v:[i64;n]
    }
    v.sort();
    let mut res = v[0] as f64;
    for i in 1..n {
        res = (res + v[i as usize] as f64) / 2.0;
    }
    println!("{}", res)
}

