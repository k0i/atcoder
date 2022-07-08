use proconio::input;

pub fn main() {
    input! {
        n:u64
    }
    print!("{}", (n - 1) * n / 2)
}
