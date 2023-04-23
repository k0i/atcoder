use proconio::{input, marker::Usize1};

fn main() {
    let n = read();

    let mut left = 0;
    let mut right = n;
    while right - left > 1 {
        let mid = (right + left) / 2;

        println!("? {}", mid + 1);
        let i = read();
        if i == 0 {
            left = mid;
        } else {
            right = mid;
        }
    }

    println!("! {}", left + 1)
}

fn read() -> usize {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}
