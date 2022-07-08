use proconio::input;
pub fn main() {
    input! {
        n:u64,
        a:u64,
        b:u64,
        s:String
    }
    let strarr: Vec<_> = s.chars().collect();
    let mut count_internal = 0;
    let mut count_external = 0;
    for i in 0..n {
        if strarr.get(i as usize).is_none() {
            break;
        }
        let target = strarr.get(i as usize).unwrap();
        if count_internal >= a + b {
            println!("No")
        } else if target.to_string() == "a" {
            count_internal += 1;
            if count_internal <= a + b {
                println!("Yes")
            } else {
                println!("No")
            }
        } else if target.to_string() == "b" {
            count_external += 1;
            if count_internal <= a + b && count_external <= b {
                count_internal += 1;
                println!("Yes")
            } else {
                println!("No")
            }
        } else {
            println!("No")
        }
    }
}
