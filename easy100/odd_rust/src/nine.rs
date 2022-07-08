use proconio::input;
pub fn main() {
    input! {
            n:i16,
            k:i16,
            b:[i16;n]
    }
    let mut res = 0;
    for i in 0..b.len() {
        if b[i] < (b[i] - k).abs() {
            res += b[i] * 2
        } else {
            res += (b[i] - k).abs() * 2
        }
    }
    print!("{}", res)
}
