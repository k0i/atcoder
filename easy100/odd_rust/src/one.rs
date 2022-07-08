use proconio::input;
pub fn main() {
    input! {
    mut a:u16,
    b:u16
    }
    let mut i = 1;
    let mut sum = a;
    while sum < b {
        sum += a - 1;
        i += 1;
    }
    if b == 1 {
        println!("{}", 0)
    } else {
        println!("{}", i)
    }
}
