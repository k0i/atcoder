use proconio::input;

pub fn main() {
    input! {
        n:usize
    }
    for i in 0..1 << n {
        let mut temp = String::new();
        for j in (0..n).rev() {
            if i & (1 << j) != 0 {
                temp.push(')');
            } else {
                temp.push('(');
            }
        }
        if valid(&temp) {
            println!("{}", temp);
            continue;
        }
    }
}

fn valid(s: &str) -> bool {
    let mut score = 0;
    let mut res = true;
    s.chars().for_each(|c| {
        if c == '(' {
            score += 1;
        } else {
            score -= 1;
            if score < 0 {
                res = false;
            }
        }
    });
    match res {
        true => score == 0,
        false => false,
    }
}
