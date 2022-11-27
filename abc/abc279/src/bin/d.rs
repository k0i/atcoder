use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
        a: f64,
        b: f64
    };
    let g = (a / 2.0 / b).powf(2.0 / 3.0).ceil();

    let mut answer = std::f64::MAX;

    for i in -5..6 {
        answer = answer.min(a / (g + i as f64).sqrt() + b * (g + i as f64 - 1.0));
    }
    println!("{}", answer)
}
