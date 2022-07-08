use proconio::input;
pub fn main() {
   input! {
        n:usize,
        mut a:[u32;n]
    }
    a.sort();
    let b = &a[n / 2 - 1..n / 2 + 1];
    println!("{:?}", b[1] - b[0]);
}
