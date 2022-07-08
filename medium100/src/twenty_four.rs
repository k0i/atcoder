use proconio::input;
pub fn main() {
    input! {
       n:u64,
       a:u64,
       b:u64,
    }
    if a > b {
        print!("{}", 0);
    } else if n == 1 {
        if a == b {
            print!("{}", 1);
        } else {
            print!("{}", 0)
        }
    } else {
        let max = (n - 1) * b + a;
        let min = (n - 1) * a + b;
        println!("{}", max - min + 1);
    }
}
