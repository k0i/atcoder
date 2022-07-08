use proconio::input;
pub fn main() {
    input! {
       n:usize,
       k:[(i64,i64,i64);n]
    }

    let mut dx = 0;
    let mut dy = 0;
    let mut dt = 0;
    'outer: for i in 0..n {
        let time = k[i].0 - dt;

        let dis = (k[i].1 - dx).abs() + (k[i].2 - dy).abs();

        if dis > time as i64 {
            println!("No");
            break 'outer;
        }
        if (time - dis) as i64 % 2 == 1 {
            println!("No");
            break 'outer;
        }
        if i == n - 1 {
            println!("Yes");
            break;
        }
        dx = k[i].1;
        dy = k[i].2;
        dt = k[i].0;
    }
}
