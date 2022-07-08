use proconio::input;
pub fn main() {
    input! {
       mut k:usize,
       a:usize,
       b:usize
    }
    if a + 2 >= b {
        println!("{}", k + 1);
    } else if k <= a - 1 {
        println!("{}", k + 1);
    } else {
        let mut res = a;
        k -= a - 1;
        if k % 2 == 1 {
            res += 1;
            k -= 1;
        }
        res += (b - a) * (k / 2);
        println!("{}", res);
    }
}
