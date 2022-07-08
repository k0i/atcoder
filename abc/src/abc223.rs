use proconio::input;
pub fn main() {
    c();
}

fn c() {
    input! {
    n:usize,
    d:[(f64,f64);n]
        }
    let mut time = 0.0;
    d.iter().for_each(|&(a, b)| {
        time += a / b;
    });
    time /= 2.0;
    let mut ans = 0.0;
    for i in 0..n {
        let t = d[i].0 / d[i].1;
        if t < time {
            time -= t;
            ans += d[i].0;
        } else {
            ans += d[i].1 * time;
            break;
        }
    }
    println!("{}", ans);
}
