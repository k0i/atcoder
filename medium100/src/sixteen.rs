use proconio::input;
pub fn main() {
    input! {
       n:usize,
       mut s:[u32;n]
    }
    s.sort();

    let mut sum: u32 = s.iter().sum();
    if sum % 10 == 0 {
        let mut i = 0;
        loop {
            sum = sum - s[i];
            if sum % 10 != 0 {
                println!("{:?}", sum);
                break;
            }
            
            if i == s.len() - 1 {
                println!("{}", 0);
                break;
            }
            i += 1;
        }
    } else {
        println!("{}", sum);
    }
}
