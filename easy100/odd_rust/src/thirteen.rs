use proconio::input;
pub fn main() {
    input! {mut a:[u64;3]}
    let mut res = 0;
    if (a[0] == a[1] && a[1] == a[2] && a[2] == a[0])
        && (a[0] % 2 == 0 && a[1] % 2 == 0 && a[2] % 2 == 0)
    {
        println!("-1")
    } else {
        loop {
            if a[0] % 2 == 0 && a[1] % 2 == 0 && a[2] % 2 == 0 {
                a = vec![
                    a[1] / 2 + a[2] / 2,
                    a[0] / 2 + a[2] / 2,
                    a[0] / 2 + a[1] / 2,
                ];
                res += 1
            } else {
                break;
            }
        }
        println!("{}", res)
    }
}
