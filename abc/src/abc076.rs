use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut start = !0;
    if s.len() < t.len() {
        println!("UNRESTORABLE");
        return;
    }
    for i in 0..=s.len() - t.len() {
        let mut flg = true;
        for j in 0..t.len() {
            if s[i + j] != '?' && s[i + j] != t[j] {
                flg = false;
                break;
            }
        }
        if flg {
            start = i;
        }
    }
    if start == !0 {
        println!("UNRESTORABLE");
        return;
    }
    let mut ans = s
        .iter()
        .map(|&x| if x == '?' { 'a' } else { x })
        .collect::<Vec<char>>();
    for (ix, c) in t.iter().enumerate() {
        ans[start + ix] = *c;
    }
    println!("{}", ans.into_iter().collect::<String>());
}
