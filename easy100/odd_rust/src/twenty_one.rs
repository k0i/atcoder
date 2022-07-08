use proconio::input;
pub fn main() {
    input! {
    mut x:i64
    }
    let res: i64;
    if x <= 2 {
        res = 2
    } else {
        'outer: loop {
            'inner: for i in 2..x {
                if x % i == 0 {
                    x += 1;
                    break 'inner;
                }
                if i+1 == x {
                    res = x;
                    break 'outer;
                }
            }
        }
    }
            println! {"{}",res};
}
