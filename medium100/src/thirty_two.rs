use proconio::input;
pub fn main() {
    input! {
       n:u64,
       y:u64
    }
    'outer:for i in 0..=n {
        for j in 0..=n {
            let k = (n as i64) - (i as i64) - (j as i64);
            if k >= 0 && i * 1000 + j * 5000 + (k * 10000) as u64 == y {
                println!("{} {} {}",k,j,i);
                break 'outer;
            }
        }
        if i == n {
            println!("-1 -1 -1");
            break 'outer;
        }
    }
}
