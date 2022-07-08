use proconio::input;
pub fn main() {
input!{
    n:i64,
    p:i64,
    x:[i64;n],
}

let mut res = (p-x[0]).abs();

for i in 1..n{
      res = gcd(res, (p-x[i as usize]).abs());
}
println!("{:?}",res);
}

fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}