use proconio::input;

fn main() {
    input! {
    x:u64,
    y:u64,
    n:u64
        }
    if n < 3 {
        println!("{:?}", x * n);
        return;
    }
    if x * 3 > y {
        let mut res = 0;
        let temp = n / 3;
        res += temp * y;
        res += x * (n - temp * 3);
        println!("{:?}", res);
    } else {
        println!("{:?}", x * n);
    }
}
