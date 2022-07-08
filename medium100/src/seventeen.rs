use proconio::input;
pub fn main() {
    input! {
         n:usize,
         time:u32,
    t:[u32;n]
      }
    let mut res = 0;
    for i in 0..n-1 {
        res += min(time, t[i+1]-t[i]);
    }
    res +=time;
    println!("{}",res)
}

fn min(a: u32, b: u32) -> u32 {
    if a < b {
        a
    } else {
        b
    }
}
