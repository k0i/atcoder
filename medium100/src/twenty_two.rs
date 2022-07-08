use proconio::input;
pub fn main() {
    input! {n:u64,
    k:u64}
    let mut res = 0.0;
    for i in 0..n {
        let mut t = 1.0 / (n as f64);
        let mut dice = i + 1;
        if dice >= k {
            res += t;
            continue;
        };
        while dice < k {
            t *= 0.5;
            dice *= 2;
        }
        res += t
    }
    println!("{}", res);
}
