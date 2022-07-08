use proconio::input;
pub fn main() {
    input! {
        n:i32,
        m:i32,
        c:i32,
        b_arr:[i32;m],
        a_arr:[[i32;m];n]
    }
    let mut res = 0;

    for i in 0..n {
        let mut sum = 0;
        for j in 0..m {
            sum += a_arr[i as usize][j as usize] * b_arr[j as usize];
        }
        if sum + c > 0 {
            res += 1
        };
    }
    println!("{}", res)
}
