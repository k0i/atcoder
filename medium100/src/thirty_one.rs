use proconio::input;
pub fn main() {
    input! {
       n:usize,
       a:[u64;n]
    }
    let mut four = 0;
    let mut none = 0;
    for i in 0..n {
        if a[i] % 4 == 0 {
            four += 1;
        } else if a[i] % 2 == 0 {
            continue;
        } else {
            none += 1;
        }
    }
    if four + 1 == none && n == four + none {
        print!("Yes");
    } else if four < none {
        print!("No");
    } else {
        print!("Yes");
    }
}
