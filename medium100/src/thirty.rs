use proconio::input;
pub fn main() {
    input! {
      mut n:usize,
       mut m:usize
    }
    let temp = n.clone();
    let temp2 = m.clone();
    if n > m {
        n = temp2;
        m = temp;
    };
    let ans = if n == 1 {
        if m == 1 {
            1
        } else {
            m - 2
        }
    } else {
        (n - 2) * (m - 2)
    };
    println!("{:?}", ans);
}
