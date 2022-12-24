fn main() {
    proconio::input! {
    s:proconio::marker::Chars
        }
    let mut a = std::collections::HashSet::new();
    for i in 0..s.len() {
        match s[i] {
            '(' => {
                continue;
            }
            ')' => {
                a.clear();
            }
            _ => {
                if a.contains(&s[i]) {
                    println!("No");
                    return;
                }
                a.insert(s[i]);
            }
        }
    }
    println!("Yes");
}
