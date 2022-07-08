use proconio::input;
pub fn main() {
    input! {
    a:u64,
    b:u64,
    c:u64
    }
    let mut devided: Vec<u64> = Vec::new();
    let mut i = 1;
    loop {
        if a * i % b == c {
            println!("YES");
            break;
        } else {
            i += 1;
            if devided
                .clone()
                .into_iter()
                .find(|x| *x == a * i % b)
                .is_some()
            {
                println!("NO");
                break;
            }
            devided.push(a * i % b);
        }
    }
}
